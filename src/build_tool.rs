use crate::cache::Cache;
use crate::{DEBUG, Lang};
use crate::rust::Project;

pub fn build_tool(lang: &Lang, command_str: &str, cache: &mut Cache) -> (bool, String) {
    match lang {
        Lang::Rust => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/lib.rs").unwrap();
            let dependencies = std::fs::read_to_string("sandbox/Cargo.toml").unwrap();
            let src= format!("{}\n{}", dependencies, code);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts= command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    let output = std::process::Command::new(command_parts[0])
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => {
                    result.to_string()
                }
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        Lang::Java => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/main/java/com/example/solution/Solution.java").unwrap();
            let test = std::fs::read_to_string("sandbox/src/test/java/com/example/solution/SolutionTest.java").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/pom.xml").unwrap();
            let src= format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts= command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => {
                    result.to_string()
                }
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }

        Lang::Scala => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/main/scala/Solution.scala").unwrap();
            let test = std::fs::read_to_string("sandbox/src/test/scala/SolutionTest.scala").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/build.sbt").unwrap();
            let src= format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts= command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => {
                    result.to_string()
                }
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }

        Lang::JavaScript => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/solution.js").unwrap();
            let test = std::fs::read_to_string("sandbox/src/solution.test.js").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/package.json").unwrap();
            let src= format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts= command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => {
                    result.to_string()
                }
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        Lang::Php => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/Solution.php").unwrap();
            let test = std::fs::read_to_string("sandbox/tests/SolutionTest.php").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/composer.json").unwrap();
            let src= format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts= command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    println!("ARGS - {:?}", args);

                    let output = std::process::Command::new(&command)
                        .args(&args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();

                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => {
                    result.to_string()
                }
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        _ => panic!("Unsupported language: {:?}", lang),

    }

}

pub fn create_project_rust(lang: &Lang, project: &Project) {
    match lang {
        Lang::Rust => {

            println!("Create sandbox project with");
            println!("{}\n{}", project.cargo_toml, project.lib_rs);
            let sandbox_path = "sandbox";
            let src_path = format!("{}/src", sandbox_path);
            let main_path = format!("{}/src/lib.rs", sandbox_path);
            let cargo_path = format!("{}/Cargo.toml", sandbox_path);
            if !std::path::Path::new(sandbox_path).exists() {
                std::fs::create_dir(sandbox_path).unwrap();
            } else {
                std::fs::remove_dir_all(sandbox_path).unwrap();
                std::fs::create_dir(sandbox_path).unwrap();
            }
            if !std::path::Path::new(&src_path).exists() {
                std::fs::create_dir(&src_path).unwrap();
            }
            std::fs::write(&main_path, &project.lib_rs).unwrap();
            std::fs::write(&cargo_path, &project.cargo_toml ).unwrap();
        }
        _ => panic!("Unsupported language: {:?}", lang),
    }
}
pub fn create_project_java(project: &crate::java::Project) {
        println!("Create sandbox project with");
        println!("{}\n{}\n{}", project.project_build_script, project.solution_code, project.test_code);
        let sandbox_path = "sandbox";

        let main_path = format!("{}/src/main/java/com/example/solution/Solution.java", sandbox_path);
        let test_path = format!("{}/src/test/java/com/example/solution/SolutionTest.java", sandbox_path);
        let pom_path = format!("{}/pom.xml", sandbox_path);
        if !std::path::Path::new(sandbox_path).exists() {
            std::fs::create_dir(sandbox_path).unwrap();
        } else {
            std::fs::remove_dir_all(sandbox_path).unwrap();
            std::fs::create_dir(sandbox_path).unwrap();
        }
        std::fs::create_dir_all(format!("{}/src/main/java/com/example/solution", sandbox_path)).unwrap();
        std::fs::create_dir_all(format!("{}/src/test/java/com/example/solution", sandbox_path)).unwrap();
        std::fs::write(&main_path, &project.solution_code).unwrap();
        std::fs::write(&test_path, &project.test_code).unwrap();
        std::fs::write(&pom_path, &project.project_build_script).unwrap();
}
pub fn create_project_scala(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!("{}\n{}\n{}", project.project_build_script, project.solution_code, project.test_code);
    let sandbox_path = "sandbox";

    let main_path = format!("{}/src/main/scala/Solution.scala", sandbox_path);
    let test_path = format!("{}/src/test/scala/SolutionTest.scala", sandbox_path);
    let pom_path = format!("{}/build.sbt", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src/main/scala", sandbox_path)).unwrap();
    std::fs::create_dir_all(format!("{}/src/test/scala", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}
pub fn create_project_javascript(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!("{}\n{}\n{}", project.project_build_script, project.solution_code, project.test_code);
    let sandbox_path = "sandbox";

    let main_path = format!("{}/src/solution.js", sandbox_path);
    let test_path = format!("{}/src/solution.test.js", sandbox_path);
    let pom_path = format!("{}/package.json", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}
pub fn create_project_php(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!("{}\n{}\n{}", project.project_build_script, project.solution_code, project.test_code);
    let sandbox_path = "sandbox";
    let main_path = format!("{}/src/Solution.php", sandbox_path);
    let test_path = format!("{}/tests/SolutionTest.php", sandbox_path);
    let pom_path = format!("{}/composer.json", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src", sandbox_path)).unwrap();
    std::fs::create_dir_all(format!("{}/tests", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}


fn only_error_message(output: &str, exit_code: i32) -> String {
    if exit_code == 0 {
        return "".to_string()
    } else {
        output.to_string()
    }
}
