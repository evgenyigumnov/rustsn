use std::collections::HashMap;
use crate::build_tool::{build_tool, create_project};
use crate::cache::Cache;
use crate::{DEBUG, MAX_NUMBER_OF_ATTEMPTS};
use crate::llm_parser::{extract, extract_number};
use crate::llm_prompt::Prompt;
use crate::llm_api::LLMApi;


pub fn run_state_machine(
    states_str_var: &str,
    question: &str,
    code: &mut String,
    dependencies: &mut String,
    tests: &mut String,
    output: &mut String,
    prompt: &Prompt,
    cache:  &mut Cache,
    llm: &LLMApi,
) {
    let states: HashMap<String, State> = extract_states(states_str_var);
    let mut current_state_name: String = extract_first_state(states_str_var);
    let mut current_state_params: HashMap<String, String> = HashMap::new();
    let mut number_of_attempts = 0;
    loop {
        if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
            println!("To many attempts");
            println!("================");
            break;
        }
        number_of_attempts += 1;
        let state_name = current_state_name.as_str();
        println!("State name: {}", current_state_name);
        println!("Current state params: {:#?}", current_state_params);
        if DEBUG {
            println!("Code: {}", code);
            println!("Dependencies: {}", dependencies);
            println!("Tests: {}", tests);
            println!("Output: {}", output);
        }
        let state_type = extract_state_type(state_name);
        let state_params = extract_state_params(state_name);
        let current_state = states.get(state_name).unwrap();
        match state_type.as_str() {
            "llm_request" => {
                let array_src = extract_param_array(state_params[1]);
                let array:Vec<String> = replace_in_array(array_src,  question, code, dependencies, tests, output, current_state_params);
                // println!("{:#?}", array);
                let result = llm.request(state_params[0].replace("\"", "").as_str(), &array, cache, prompt);

                let next_state_name = current_state.transitions.keys().next().unwrap().to_string();
                let param = current_state.transitions.get(&next_state_name).unwrap().to_string();
                let mut next_state_params = HashMap::new();
                next_state_params.insert(param, result);

                current_state_name = next_state_name;
                current_state_params = next_state_params;
                println!("===============");

                continue;
            }
            "extract_code" | "extract_dep" | "extract_test" => {
                let extract_type = match state_type.as_str() {
                    "extract_code" => "code",
                    "extract_dep" => "dependencies",
                    "extract_test" => "tests",
                    &_ => panic!("Unknown extract type: {}", state_type),
                };
                let result = extract(current_state_params.get(state_params[0]).unwrap(), extract_type);
                let next_state_name = current_state.transitions.keys().next().unwrap().to_string();
                let param = current_state.transitions.get(&next_state_name).unwrap().to_string();
                let mut next_state_params = HashMap::new();
                next_state_params.insert(param.clone(), result.clone());


                current_state_name = next_state_name;
                current_state_params = next_state_params;
                update_global_vars(&param, &result, code, dependencies, tests, output);
                println!("===============");
                continue;
            }
            "create_project" => {
                if current_state_params.contains_key("code") {
                    let code_param = current_state_params.get("code").unwrap();
                    update_global_vars("code", code_param, code, dependencies, tests, output);
                } else {
                    if !state_params.contains(&"code") {
                        update_global_vars("code", "", code, dependencies, tests, output);
                    }
                }
                if current_state_params.contains_key("dependencies") {
                    let dependencies_param = current_state_params.get("dependencies").unwrap();
                    update_global_vars("dependencies", dependencies_param, code, dependencies, tests, output);
                } else {
                    if !state_params.contains(&"dependencies") {
                        update_global_vars("dependencies", "", code, dependencies, tests, output);
                    }
                }
                if current_state_params.contains_key("tests") {
                    let tests_param = current_state_params.get("tests").unwrap();
                    update_global_vars("tests", tests_param, code, dependencies, tests, output);
                } else {
                    if !state_params.contains(&"tests") {
                        update_global_vars("tests", "", code, dependencies, tests, output);
                    }
                }

                create_project(code, dependencies, tests);
                let next_state_name = current_state.transitions.keys().next().unwrap().to_string();
                current_state_name = next_state_name;
                current_state_params = HashMap::new();
                println!("===============");
                continue;
            }
            "build_tool" => {
                let result:(bool, String) = build_tool(&state_params[0].replace("\"",""), cache);
                let param_first_name = result.0.to_string();
                let param_first_name_value = result.0.to_string();
                let param_second_name = "output".to_string();
                let param_second_value = result.1.to_string();
                update_global_vars("output", &param_second_value, code, dependencies, tests, output);
                let transition_condition = format!("({},{})", param_first_name, param_second_name);
                // println!("{}", transition_condition);
                let mut next_state_name = "".to_string();
                for (key, value) in current_state.transitions.iter() {
                    if value == &transition_condition {
                        next_state_name = key.to_string();
                        break;
                    }
                }
                let mut next_state_params = HashMap::new();
                next_state_params.insert(param_first_name, param_first_name_value);
                next_state_params.insert(param_second_name, param_second_value);
                current_state_name = next_state_name;
                current_state_params = next_state_params;
                println!("===============");
                continue;
            }
            "extract_number" => {
                let result = extract_number(current_state_params.get(state_params[0]).unwrap()).to_string();
                let mut next_state_name: String = "".to_string();
                for (key, value) in current_state.transitions.iter() {
                    next_state_name = if value == &result {
                        key.to_string()
                    } else {
                        continue;
                    };
                }
                if &next_state_name == "" {
                    panic!("Transition not found");
                }
                let mut next_state_params = HashMap::new();
                next_state_params.insert(result.to_string(), result.to_string());

                current_state_name = next_state_name;
                current_state_params = next_state_params;
                println!("===============");
                continue;
            }
            "finish" => {
                return;
            }
            &_ => {

                current_state_params = HashMap::new();
                current_state_name = "finish".to_string();
                println!("===============");
                continue;
            }
        }
    }
}

fn replace_in_array(array: Vec<&str>, question: &str, code: &str, dependencies: &str, tests: &str, output: &str ,params: HashMap<String, String>) -> Vec<String> {
    let mut new_array = Vec::new();
    for item in array {
        match item {
            "question" => new_array.push(question.to_string()),
            "code" => new_array.push(code.to_string()),
            "dependencies" => new_array.push(dependencies.to_string()),
            "tests" => new_array.push(tests.to_string()),
            "output" => new_array.push(output.to_string()),
            &_ => {
                if params.contains_key(item) {
                    new_array.push(params.get(item).unwrap().to_string())
                }
            }
        }
    }
    new_array
}
fn update_global_vars(param_name: &str, param_value: &str, code: &mut String, dependencies: &mut String, tests: &mut String, output: &mut String)  {
    match param_name {
        "code" => *code = param_value.to_string(),
        "dependencies" => *dependencies = param_value.to_string(),
        "tests" => *tests = param_value.to_string(),
        "output" => *output = param_value.to_string(),
        &_ => {}
    }
}


fn extract_first_state(states_str_var: &str) -> String {
    let mut states = extract_states_impl(states_str_var);
    let first_state = states.remove("[*]").unwrap();
    first_state.transitions.keys().next().unwrap().to_string()
}
#[derive(Debug)]
pub struct State {
    #[allow(dead_code)]
    name: String,
    transitions: HashMap<String, String>, // state_name, condition
}
fn extract_states(states_str_var: &str) -> HashMap<String, State> {
    let mut states = extract_states_impl(states_str_var);
    states.retain(|k, _| k != "[*]");
    states
}

fn extract_states_impl(states_str_var: &str) -> HashMap<String, State> {
    let mut states_map = HashMap::new();

    for line in states_str_var.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let line = line.trim_start_matches("//").trim();

        let parts: Vec<&str> = line.split("-->").collect();
        if parts.len() != 2 {
            continue;
        }
        let source = parts[0].trim();
        let rest = parts[1].trim();

        let target_and_condition: Vec<&str> = rest.split(':').collect();
        let target = target_and_condition[0].trim();
        let condition = if target_and_condition.len() > 1 {
            target_and_condition[1].trim()
        } else {
            ""
        };

        let source_state = states_map.entry(source.to_string()).or_insert(State {
            name: source.to_string(),
            transitions: HashMap::new(),
        });
        source_state
            .transitions
            .insert(target.to_string(), condition.to_string());

        // Ensure the target state exists in the map
        states_map.entry(target.to_string()).or_insert(State {
            name: target.to_string(),
            transitions: HashMap::new(),
        });
    }

    states_map
}

fn extract_state_type(state_str: &str) -> String {
    let state_type = state_str.split("(").collect::<Vec<&str>>()[0];
    state_type.to_string()
}
fn extract_state_params(state_str: &str) -> Vec<&str> {
    if let Some(start) = state_str.find('(') {
        if let Some(end) = state_str.rfind(')') {
            let params_str = &state_str[start + 1..end];
            let mut params = Vec::new();
            let mut current = 0;
            let mut in_quotes = false;
            let mut bracket_depth = 0;

            for (i, c) in params_str.char_indices() {
                match c {
                    '"' => {
                        in_quotes = !in_quotes;
                    },
                    '[' => {
                        if !in_quotes {
                            bracket_depth += 1;
                        }
                    },
                    ']' => {
                        if !in_quotes && bracket_depth > 0 {
                            bracket_depth -= 1;
                        }
                    },
                    ',' => {
                        if !in_quotes && bracket_depth == 0 {
                            // Split here
                            params.push(params_str[current..i].trim());
                            current = i + 1;
                        }
                    },
                    _ => {}
                }
            }
            // Push the last parameter
            params.push(params_str[current..].trim());

            params
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}
fn extract_param_array(param_str: &str) -> Vec<&str> {
    let state_params = param_str.split("[").collect::<Vec<&str>>()[1];
    let state_params = state_params.split("]").collect::<Vec<&str>>()[0];
    state_params.split(",").collect::<Vec<&str>>()
}

mod tests {
    const _STATES: &str = r#"
```mermaid
stateDiagram
[*] --> llm_request("generate_code_prompt_template",[question]) : question
llm_request("generate_code_prompt_template",[question]) --> extract_code(code_response) : code_response
extract_code(code_response) --> create_project(code,dependencies,tests) : code
create_project(code,dependencies,tests) --> build_tool("build")
build_tool("build") --> finish : (true,output)
build_tool("build") --> llm_request("build_dependencies_req_prompt_template",[question,code,output]) : (false,output)
llm_request("build_dependencies_req_prompt_template",[question,code,output])  --> extract_number(response) : response
extract_number(response) --> finish : 2
extract_number(response) --> llm_request("build_dependencies_prompt_template",[question,code]) : 1
llm_request("build_dependencies_prompt_template",[question,code]) --> extract_code(dependencies_response) : dependencies_response
extract_code(dependencies_response) --> create_project(code,dependencies,tests) : dependencies
finish --> [*]
```
"#;
    #[test]
    fn test_extract_state_type() {
        let state_str = r#"llm_request("generate_code_prompt_template",[question])"#;
        assert_eq!(super::extract_state_type(state_str), "llm_request");
    }

    #[test]
    fn test_extract_state_params() {
        let state_str = r#"llm_request("build_dependencies_req_prompt_template",abc,[question,code,output])"#;
        assert_eq!(super::extract_state_params(state_str), vec!["\"build_dependencies_req_prompt_template\"","abc","[question,code,output]"]);
    }

    #[test]
    fn test_extract_param_array() {
        let param_str = "[question,code,output]";
        assert_eq!(super::extract_param_array(param_str), vec!["question","code","output"]);
    }

    #[test]
    fn test_extract_states() {
        println!("{:#?}",  crate::state_machine::extract_states(_STATES));
    }

    #[test]
    fn test_extract_first_state() {
        let first_state = super::extract_first_state(_STATES);
        assert_eq!(first_state, "llm_request(\"generate_code_prompt_template\",[question])");
    }

}