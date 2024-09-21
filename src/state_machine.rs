use crate::build_tool::{build_tool, create_project, create_project_java};
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
    match lang {
        Lang::Rust => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
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
            if build_res.0 && test_res.0 {
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
                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::Java => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = crate::java::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_java(&project);
            println!("================");
            let mut build_res = build_tool(lang, &project.build, cache);
            println!("================");
            let mut test_res = build_tool(lang, &project.test, cache);
            println!("================");
            if build_res.0 && test_res.0 {
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
                                             &vec![project.pom_xml,
                                                   project.solution_java,
                                                   project.test_java,
                                                   project.build,
                                                   build_res.1,
                                                   project.test,
                                                   test_res.1,
                                                   question.to_string()],
                                             cache, prompt);
                    project = crate::java::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_java(&project);
                    println!("================");
                    build_res = build_tool(lang, &project.build, cache);
                    println!("================");
                    test_res = build_tool(lang, &project.test, cache);
                    println!("================");
                    if build_res.0 && test_res.0 {
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

