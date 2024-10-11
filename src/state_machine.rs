use crate::build_tool::{
    build_tool, create_project_java, create_project_javascript, create_project_kotlin,
    create_project_php, create_project_python, create_project_rust, create_project_scala,
    create_project_swift, create_project_typescript,
};
use crate::cache::Cache;
use crate::llm_api::LLMApi;
use crate::llm_prompt::Prompt;
use crate::llm_response::LLMResponse;
use crate::{Lang, MAX_NUMBER_OF_ATTEMPTS, VERBOSE};

pub fn run_state_machine(
    lang: &Lang,
    question: &str,
    prompt: &Prompt,
    cache: &mut Cache,
    llm: &LLMApi,
) {
    match lang {
        Lang::Rust => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::Rust);
            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }
            create_project_rust(lang, &project);
            let mut build_res = build_tool(lang, &project.build_command, cache);
            let mut test_res = build_tool(lang, &project.test_command, cache);
            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");
                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.build_command,
                            build_res.1,
                            project.test_code,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::Rust);
                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }
                    create_project_rust(lang, &project);
                    build_res = build_tool(lang, &project.build_command, cache);
                    test_res = build_tool(lang, &project.test_command, cache);
                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::Java => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::Java);
            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }
            create_project_java(&project);
            let mut build_res = build_tool(lang, &project.build_command, cache);
            let mut test_res = build_tool(lang, &project.test_command, cache);
            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");
                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::Java);

                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }

                    create_project_java(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::Scala => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::Scala);

            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }

            create_project_scala(&project);

            let mut build_res = build_tool(lang, &project.build_command, cache);

            let mut test_res = build_tool(lang, &project.test_command, cache);

            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");

                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::Scala);

                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }

                    create_project_scala(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::Swift => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::Swift);

            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }

            create_project_swift(&project);

            let mut build_res = build_tool(lang, &project.build_command, cache);

            let mut test_res = build_tool(lang, &project.test_command, cache);

            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");

                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::Swift);

                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }

                    create_project_swift(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::Kotlin => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::Kotlin);

            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }

            create_project_kotlin(&project);

            let mut build_res = build_tool(lang, &project.build_command, cache);

            let mut test_res = build_tool(lang, &project.test_command, cache);

            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");

                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::Kotlin);

                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }

                    create_project_kotlin(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::Python => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::Python);

            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }

            create_project_python(&project);

            let mut build_res = build_tool(lang, &project.build_command, cache);

            let mut test_res = build_tool(lang, &project.test_command, cache);

            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");

                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::Python);

                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }

                    create_project_python(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::JavaScript => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::JavaScript);

            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }

            if *VERBOSE.lock().unwrap() {
                create_project_javascript(&project);
            }

            let mut build_res = build_tool(lang, &project.build_command, cache);

            let mut test_res = build_tool(lang, &project.test_command, cache);

            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");

                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::JavaScript);

                    println!("{:#?}", project);

                    create_project_javascript(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::TypeScript => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::TypeScript);

            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }

            create_project_typescript(&project);

            let mut build_res = build_tool(lang, &project.build_command, cache);

            let mut test_res = build_tool(lang, &project.test_command, cache);

            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    let ts_config = project.additional_config[0].clone();
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");

                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            ts_config,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::TypeScript);

                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }

                    create_project_typescript(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

                    if build_res.0 && test_res.0 {
                        return;
                    }
                }
            }
        }
        Lang::Php => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = LLMResponse::parse_llm_response(&result, Lang::Php);

            if *VERBOSE.lock().unwrap() {
                println!("{:#?}", project);
            }

            create_project_php(&project);

            let mut build_res = build_tool(lang, &project.build_command, cache);

            let mut test_res = build_tool(lang, &project.test_command, cache);

            if build_res.0 && test_res.0 {
                return;
            } else {
                let mut number_of_attempts = 0;
                loop {
                    if number_of_attempts > MAX_NUMBER_OF_ATTEMPTS {
                        println!("To many attempts");

                        break;
                    }
                    number_of_attempts += 1;
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.dependencies,
                            project.solution_code,
                            project.test_code,
                            project.build_command,
                            build_res.1,
                            project.test_command,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = LLMResponse::parse_llm_response(&result, Lang::Php);

                    if *VERBOSE.lock().unwrap() {
                        println!("{:#?}", project);
                    }

                    create_project_php(&project);

                    build_res = build_tool(lang, &project.build_command, cache);

                    test_res = build_tool(lang, &project.test_command, cache);

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
