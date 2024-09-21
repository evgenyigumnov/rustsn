use crate::build_tool::{build_tool, create_project};
use crate::cache::Cache;
use crate::{Lang, MAX_NUMBER_OF_ATTEMPTS};
use crate::llm_prompt::Prompt;
use crate::llm_api::LLMApi;


pub fn run_state_machine(
    lang: &Lang,
    question: &str,
    prompt: &Prompt,
    cache:  &mut Cache,
    llm: &LLMApi,
) {
    let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
    match lang {
        Lang::Rust => {
            let mut project = crate::rust::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project(lang, &project);
            println!("================");
            let mut build_res = build_tool(lang, &project.build, cache);
            println!("================");
            let mut test_res = build_tool(lang, &project.test, cache);
            println!("================");
            if build_res.0 || test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");
                        println!("================");
                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request("rewrite",
                                             &vec![project.cargo_toml,
                                                   project.lib_rs,
                                                   project.build,
                                                   build_res.1,
                                                   project.test,
                                                   test_res.1,
                                                   question.to_string()],
                                             cache, prompt);
                    project = crate::rust::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project(lang, &project);
                    println!("================");
                    build_res = build_tool(lang, &project.build, cache);
                    println!("================");
                    test_res = build_tool(lang, &project.test, cache);
                    println!("================");
                    if build_res.0 || test_res.0 {
                        return;
                    }
                }
            }
        }
        &_ => {
            panic!("Unknown lang: {}", lang);
        }
    }

}

