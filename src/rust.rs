use regex::Regex;

#[derive(Debug)]
pub struct Project {
    pub cargo_toml: String,
    pub lib_rs: String,
    pub build: String,
    pub test: String,
}

pub fn parse_llm_response(response: &str) -> Project {
    let mut cargo_toml = String::new();
    let mut lib_rs = String::new();
    let mut build = String::new();
    let mut test = String::new();

    let re_section =
        Regex::new(r"(?m)^(?:\s*(?:\#*)?\s*\*\*)?(?:\d+\.\s*)?(.*?)[:\*]*\*\*\s*$").unwrap();

    let mut positions = Vec::new();

    for cap in re_section.captures_iter(response) {
        let section_name = cap.get(1).unwrap().as_str();
        let start = cap.get(0).unwrap().end();

        positions.push((section_name.to_string(), start));
    }

    positions.push(("".to_string(), response.len()));

    for i in 0..positions.len() - 1 {
        let (section_name, start) = &positions[i];
        let end = positions[i + 1].1;

        let section_content = &response[*start..end];

        // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
        let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

        if let Some(cap) = re_code_block.captures(section_content) {
            let content = cap.get(1).unwrap().as_str().to_string();

            if section_name.contains("Cargo.toml") {
                cargo_toml = content;
            } else if section_name.contains("src/lib.rs") {
                lib_rs = content;
            } else if section_name.contains("Build") {
                build = content;
            } else if section_name.contains("Test") {
                test = content;
            }
        }
    }

    if cargo_toml == "" {
        let mut lines = response.lines().peekable();

        while let Some(line) = lines.next() {
            if line.starts_with("## ") {
                let section_title = line[3..].trim();

                // Skip lines until the start of the code block
                while let Some(line) = lines.next() {
                    if line.starts_with("```") {
                        // Capture the code block content
                        let mut code_content = String::new();
                        while let Some(line) = lines.next() {
                            if line.starts_with("```") {
                                // End of code block
                                break;
                            } else {
                                code_content.push_str(line);
                                code_content.push('\n');
                            }
                        }

                        // Assign the captured content to the appropriate field
                        match section_title {
                            "Cargo.toml" => cargo_toml = code_content.trim_end().to_string(),
                            "src/lib.rs" => lib_rs = code_content.trim_end().to_string(),
                            "Build" => build = code_content.trim_end().to_string(),
                            "Test" => test = code_content.trim_end().to_string(),
                            _ => (),
                        }

                        // Break out of the inner loop to process the next section
                        break;
                    }
                }
            }
        }

        Project {
            cargo_toml,
            lib_rs,
            build: remove_comments(&build),
            test: remove_comments(&test),
        }
    } else {
        Project {
            cargo_toml,
            lib_rs,
            build: remove_comments(&build),
            test: remove_comments(&test),
        }
    }
}

pub fn remove_comments(text: &str) -> String {
    let re_comment = Regex::new(r"(?m)^#.*$").unwrap();
    re_comment
        .replace_all(text, "")
        .to_string()
        .trim()
        .to_string()
}
mod tests {

    #[test]
    fn test_parse_llm_response() {
        for i in 1..=8 {
            let file = format!("./test_data/rust_create_{}.txt", i);
            let response = std::fs::read_to_string(file).unwrap();
            let mut project = crate::rust::parse_llm_response(&response);
            project.build = crate::rust::remove_comments(&project.build);
            project.test = crate::rust::remove_comments(&project.test);

            println!("{:#?}", project);
            assert!(!project.cargo_toml.is_empty());
            assert!(!project.lib_rs.is_empty());
            assert!(!project.build.is_empty());
            assert!(!project.test.is_empty());
        }
    }
}
