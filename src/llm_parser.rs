use crate::DEBUG;

pub fn extract(input: &str, extract_type: &str) -> String {
    let extract = extract_impl(input);
    let res = match extract_type {
        "code" => {
            extract.code.unwrap_or("Error: extract_code()".to_string())
        },
        "dependencies" => {
            extract.dependencies.unwrap_or("Error: extract_dependencies()".to_string())
        },
        "tests" => {
            extract.tests.unwrap_or("Error: extract_tests()".to_string())
        },
        _ => {
            panic!("Unknown extract type: {}", extract_type);
        }
    };

    if DEBUG {
        println!("{}",res);
        println!("============");
    }

    res
}

#[derive(PartialEq, Debug)]
pub struct Extract {
    pub code: Option<String>,
    pub dependencies: Option<String>,
    pub tests: Option<String>,
}
pub fn extract_impl(input: &str) -> Extract {
    let mut extract = Extract {
        code: None,
        dependencies: None,
        tests: None,
    };
    let mut code = "".to_string();
    let mut in_code_block = false;
    for line in input.lines() {
        if line.trim().starts_with("```") {
            if in_code_block {
                let res = code;
                update_extract(&res, &mut extract);
                code = "".to_string();
            }
            in_code_block = !in_code_block;
        } else if in_code_block {
            code.push_str(line);
            code.push_str("\n");
        }
    }
    update_extract(&code, &mut extract);
    extract
}

pub fn update_extract(input: &str, extract: &mut Extract) {
    let input_lower = input.to_lowercase();
    let mut result = "code";
    if input_lower.contains("dependenc") {
        result = "dependencies"
    } else if input_lower.contains("test") {
        result = "tests"
    }
    match result {
        "code" => {
            if extract.code.is_none() {
                extract.code = Some(input.to_string());
            }
        },
        "dependencies" => {
            extract.dependencies = Some(input.to_string());
        },
        "tests" => {
            extract.tests = Some(input.to_string());
        },
        _ => {
            panic!("Unknown extract type: {}", result);
        }
    }
}



pub fn extract_number(input: &str) -> i32 {
    for word in input.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            return num;
        }
    }
    1 // default value if no number found
}

#[cfg(test)]
mod tests {
    use super::*;




    #[test]
    fn test_extract_number() {
        let input = "Bla bla bla\nTututu 123\nmore bla bla\nTutu 456\nbla bla";
        let expected = 123;
        assert_eq!(extract_number(input), expected);
    }

    #[test]
    fn test_extract_impl() {
        let input = r#"

```toml
[dependencies]
serde_json = "1.0.108"
```

```rust
fn main() {
}
```

```rust
mod test {
    #[test]
    fn test() {
    }
}
```
        "#;

        let expected = Extract {
            code: Some("fn main() {\n}\n".to_string()),
            dependencies: Some("[dependencies]\nserde_json = \"1.0.108\"\n".to_string()),
            tests: Some("mod test {\n    #[test]\n    fn test() {\n    }\n}\n".to_string()),
        };
        assert_eq!(extract_impl(input), expected);
    }
}