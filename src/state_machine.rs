use crate::build_tool::{
    build_tool, create_project_java, create_project_javascript, create_project_kotlin,
    create_project_php, create_project_python, create_project_rust, create_project_scala,
    create_project_swift,
};
use crate::cache::Cache;
use crate::llm_api::LLMApi;
use crate::llm_prompt::Prompt;
use crate::{Lang, MAX_NUMBER_OF_ATTEMPTS};

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
            let mut project = crate::rust::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_rust(lang, &project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.cargo_toml,
                            project.lib_rs,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = crate::rust::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_rust(lang, &project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.project_build_script,
                            project.solution_code,
                            project.test_code,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
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
        Lang::Scala => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = crate::scala::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_scala(&project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.project_build_script,
                            project.solution_code,
                            project.test_code,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = crate::scala::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_scala(&project);
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
        Lang::Swift => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = crate::swift::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_swift(&project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.project_build_script,
                            project.solution_code,
                            project.test_code,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = crate::swift::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_swift(&project);
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
        Lang::Kotlin => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = crate::kotlin::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_kotlin(&project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.project_build_script,
                            project.solution_code,
                            project.test_code,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = crate::kotlin::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_kotlin(&project);
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
        Lang::Python => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = crate::python::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_python(&project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.project_build_script,
                            project.solution_code,
                            project.test_code,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = crate::python::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_python(&project);
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
        Lang::JavaScript => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = crate::javascript::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_javascript(&project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.project_build_script,
                            project.solution_code,
                            project.test_code,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = crate::javascript::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_javascript(&project);
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
        Lang::Php => {
            let result = llm.request("generate", &vec![question.to_string()], cache, prompt);
            let mut project = crate::php::parse_llm_response(&result);
            println!("================");
            println!("{:#?}", project);
            println!("================");
            create_project_php(&project);
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
                    let result = llm.request(
                        "rewrite",
                        &vec![
                            project.project_build_script,
                            project.solution_code,
                            project.test_code,
                            project.build,
                            build_res.1,
                            project.test,
                            test_res.1,
                            question.to_string(),
                        ],
                        cache,
                        prompt,
                    );
                    project = crate::php::parse_llm_response(&result);
                    println!("================");
                    println!("{:#?}", project);
                    println!("================");
                    create_project_php(&project);
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
