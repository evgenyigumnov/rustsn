use crate::cache::Cache;
use crate::{DEBUG};

pub fn build_tool(command_str: &str, cache: &mut Cache) -> (bool, String) {
    let command = if command_str == "build_tests" {
        "test --no-run"
    } else {
        command_str
    };
    println!("Exec: cargo {}", command);
    let code = if std::path::Path::new("sandbox/src/main.rs").exists() {
        std::fs::read_to_string("sandbox/src/main.rs").unwrap()
    } else {
        "".to_string()
    };
    let dependencies = if std::path::Path::new("sandbox/Cargo.toml").exists() {
        std::fs::read_to_string("sandbox/Cargo.toml").unwrap()
    } else {
        "".to_string()
    };
    let src= format!("{}\n{}", dependencies, code);

    let key = format!("{}{}", command, src);
    let result_str_opt = cache.get(&key);
    let result_str = match result_str_opt {
        None => {
            // split by ' '
            let args= command.split(" ").collect::<Vec<&str>>();
            let output = std::process::Command::new("cargo")
                .args(args)
                .current_dir("sandbox")
                .output()
                .unwrap();
            let exit_code = output.status.code().unwrap();
            let std_out = String::from_utf8(output.stdout).unwrap();
            let std_err = String::from_utf8(output.stderr).unwrap();
            let output = std_err + &std_out;
            let tuple: (i32, String) = (exit_code, output);
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
    (exit_code_bool,extract_error_message(&output, exit_code))
}

pub fn create_project(code: &str, dependencies: &str, tests: &str) {
    let code_str = if code == "" {
        ""
    } else {
        "'code'"
    };
    let test_str = if tests == "" {
        ""
    } else {
        "'test'"
    };

    let dependencies_str = if dependencies == "" {
        ""
    } else {
        "'dependencies'"
    };

    println!("Create sandbox project with: {} {} {}", code_str,  dependencies_str, test_str);
    println!("{}\n{}\n{}", dependencies, code, tests);
    let sandbox_path = "sandbox";
    let src_path = format!("{}/src", sandbox_path);
    let main_path = format!("{}/src/main.rs", sandbox_path);
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
    let main_rs = r#"fn main() {}"#;
    std::fs::write(&main_path, format!("{}\n{}\n{}", main_rs, code, tests)).unwrap();

    std::fs::write(&cargo_path, format!(r#"
[package]
name = "sandbox"
version = "0.1.0"
edition = "2018"

{}
"#, dependencies )).unwrap();

}


fn extract_error_message(output: &str, exit_code: i32) -> String {
    let mut error_lines = Vec::new();
    let mut in_error_section = false;

    for line in output.lines() {
        if line.starts_with("error[") {
            in_error_section = true;
        }

        if in_error_section {
            error_lines.push(line);

            if line.starts_with("For more information about this error") {
                in_error_section = false;
            }
        }
    }

    let r = error_lines.join("\n");
    let ret = if r == "" && exit_code != 0 {
        output.to_string()
    } else  {
        r
    };

    if DEBUG {
        println!("=========Errors=========:");
        println!("{}", ret);
        println!("===================");
    }
    ret
}