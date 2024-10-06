use regex::Regex;

use crate::{utils::remove_comments, Lang};

#[derive(Debug)]
pub struct Project {
    pub dependencies: String,
    pub additional_config: Vec<String>,
    pub solution_code: String,
    pub test_code: String,
    pub install_dependency_command: Option<String>,
    pub build_command: String,
    pub test_command: String,
    pub lang: Lang,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            dependencies: String::new(),
            additional_config: vec![],
            solution_code: String::new(),
            test_code: String::new(),
            install_dependency_command: None,
            build_command: String::new(),
            test_command: String::new(),
            lang: Lang::Unknown,
        }
    }
}

pub struct LLMResponse;

impl LLMResponse {
    fn parse_positions(response: &str) -> Vec<(String, usize)> {
        let re_section =
            Regex::new(r"(?m)^(?:\s*(?:\#*)?\s*\*\*)?(?:\d+\.\s*)?(.*?)[:\*]*\*\*\s*$").unwrap();

        let mut positions = Vec::new();

        for cap in re_section.captures_iter(response) {
            let section_name = cap.get(1).unwrap().as_str();
            let start = cap.get(0).unwrap().end();

            positions.push((section_name.to_string(), start));
        }

        positions.push(("".to_string(), response.len()));
        positions
    }

    pub fn parse_llm_response(response: &str, language: Lang) -> Project {
        match language {
            Lang::Rust => {
                let mut cargo_toml = String::new();
                let mut lib_rs = String::new();
                let mut build = String::new();
                let mut test = String::new();

                let positions = LLMResponse::parse_positions(response);

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
                                        "Cargo.toml" => {
                                            cargo_toml = code_content.trim_end().to_string()
                                        }
                                        "src/lib.rs" => {
                                            lib_rs = code_content.trim_end().to_string()
                                        }
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
                }
                Project {
                    dependencies: cargo_toml,
                    solution_code: lib_rs,
                    build_command: remove_comments(&build),
                    test_command: remove_comments(&test),
                    lang: Lang::Rust,
                    ..Default::default()
                }
            }
            Lang::JavaScript => {
                let mut pkg_json = String::new();
                let mut solution = String::new();
                let mut install_dependency_command = String::new();
                let mut test = String::new();
                let mut test_js = String::new();

                let positions = LLMResponse::parse_positions(response);

                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();

                        if section_name.contains("package.json") {
                            pkg_json = content;
                        } else if section_name.contains("src/solution.js") {
                            solution = content;
                        } else if section_name.contains("src/solution.test.js") {
                            test_js = content;
                        } else if section_name.contains("Install") {
                            install_dependency_command = content;
                        } else if section_name.contains("Test") {
                            test = content;
                        }
                    }
                }

                if pkg_json == "" {
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
                                        "package.json" => {
                                            pkg_json = code_content.trim_end().to_string()
                                        }
                                        "src/solution.js" => {
                                            solution = code_content.trim_end().to_string()
                                        }
                                        "src/solution.test.js" => {
                                            test = code_content.trim_end().to_string()
                                        }
                                        "Install" => {
                                            install_dependency_command =
                                                code_content.trim_end().to_string()
                                        }
                                        "Test" => test = code_content.trim_end().to_string(),
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }

                Project {
                    dependencies: pkg_json,
                    solution_code: solution,
                    test_code: test_js,
                    test_command: test,
                    install_dependency_command: Some(install_dependency_command),
                    lang: Lang::JavaScript,
                    ..Default::default()
                }
            }
            Lang::Java => {
                let mut pom_xml = String::new();
                let mut solution_java = String::new();
                let mut test_java = String::new();
                let mut build_command = String::new();
                let mut test_command = String::new();

                let positions = LLMResponse::parse_positions(response);

                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();

                        if section_name.contains("pom.xml") {
                            pom_xml = content;
                        } else if section_name
                            .contains("src/main/java/com/example/solution/Solution.java")
                        {
                            solution_java = content;
                        } else if section_name
                            .contains("src/test/java/com/example/solution/SolutionTest.java")
                        {
                            test_java = content;
                        } else if section_name.contains("Compile") {
                            build_command = content;
                        } else if section_name.contains("Test") {
                            test_command = content;
                        }
                    }
                }

                if pom_xml == "" {
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
                                        "pom.xml" => pom_xml = code_content.trim_end().to_string(),
                                        "src/main/java/com/example/solution/Solution.java" => {
                                            solution_java = code_content.trim_end().to_string()
                                        }
                                        "src/test/java/com/example/solution/SolutionTest.java" => {
                                            test_java = code_content.trim_end().to_string()
                                        }
                                        "Compile" => {
                                            build_command = code_content.trim_end().to_string()
                                        }
                                        "Test" => {
                                            test_command = code_content.trim_end().to_string()
                                        }
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }

                Project {
                    dependencies: pom_xml,
                    solution_code: solution_java,
                    test_code: test_java,
                    build_command,
                    test_command,
                    ..Default::default()
                }
            }
            Lang::Kotlin => {
                let mut gradle = String::new();
                let mut solution_kotlin = String::new();
                let mut test_kotlin = String::new();
                let mut build_command = String::new();
                let mut test_command = String::new();

                let positions = LLMResponse::parse_positions(response);

                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();

                        if section_name.contains("build.gradle") {
                            gradle = content;
                        } else if section_name.contains("src/main/kotlin/Solution.kt") {
                            solution_kotlin = content;
                        } else if section_name.contains("src/test/kotlin/SolutionTest.kt") {
                            test_kotlin = content;
                        } else if section_name.contains("Compile") {
                            build_command = content;
                        } else if section_name.contains("Test") {
                            test_command = content;
                        }
                    }
                }

                if gradle == "" {
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
                                        "build.gradle" => {
                                            gradle = code_content.trim_end().to_string()
                                        }
                                        "src/main/kotlin/Solution.kt" => {
                                            solution_kotlin = code_content.trim_end().to_string()
                                        }
                                        "src/test/kotlin/SolutionTest.kt" => {
                                            test_kotlin = code_content.trim_end().to_string()
                                        }
                                        "Compile" => {
                                            build_command = code_content.trim_end().to_string()
                                        }
                                        "Test" => {
                                            test_command = code_content.trim_end().to_string()
                                        }
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }
                Project {
                    dependencies: gradle,
                    solution_code: solution_kotlin,
                    test_code: test_kotlin,
                    build_command,
                    test_command,
                    lang: Lang::Kotlin,

                    ..Default::default()
                }
            }
            Lang::Php => {
                let mut composer = String::new();
                let mut solution_php = String::new();
                let mut test_php = String::new();
                let mut install_dependency_command = String::new();
                let mut test_command = String::new();

                let positions = LLMResponse::parse_positions(response);

                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();
                        if section_name.contains("composer.json") {
                            composer = content;
                        } else if section_name.contains("src/Solution.php") {
                            solution_php = content;
                        } else if section_name.contains("tests/SolutionTest.php") {
                            test_php = content;
                        } else if section_name.contains("Install") {
                            install_dependency_command = content;
                        } else if section_name.contains("Test") {
                            test_command = content;
                        }
                    }
                }

                if composer == "" {
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
                                        "composer.json" => {
                                            composer = code_content.trim_end().to_string()
                                        }
                                        "src/Solution.php" => {
                                            solution_php = code_content.trim_end().to_string()
                                        }
                                        "tests/SolutionTest.php" => {
                                            test_php = code_content.trim_end().to_string()
                                        }
                                        "Install" => {
                                            install_dependency_command =
                                                code_content.trim_end().to_string()
                                        }
                                        "Test" => {
                                            test_command = code_content.trim_end().to_string()
                                        }
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }
                Project {
                    install_dependency_command: Some(install_dependency_command),
                    dependencies: composer,
                    solution_code: solution_php,
                    test_code: test_php,
                    test_command,
                    lang: Lang::Php,
                    ..Default::default()
                }
            }
            Lang::Python => {
                let mut requirements = String::new();
                let mut solution_py = String::new();
                let mut test_py = String::new();
                let mut install_dependency_command = String::new();
                let mut test = String::new();

                let positions = LLMResponse::parse_positions(response);

                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();

                        if section_name.contains("requirements.txt") {
                            requirements = content;
                        } else if section_name.contains("solution.py") {
                            solution_py = content;
                        } else if section_name.contains("test.py") {
                            test_py = content;
                        } else if section_name.contains("Dependencies") {
                            install_dependency_command = content;
                        } else if section_name.contains("Test") {
                            test = content;
                        }
                    }
                }

                if requirements == "" {
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
                                        "requirements.txt" => {
                                            requirements = code_content.trim_end().to_string()
                                        }
                                        "solution.py" => {
                                            solution_py = code_content.trim_end().to_string()
                                        }
                                        "test.py" => test_py = code_content.trim_end().to_string(),
                                        "Dependencies" => {
                                            install_dependency_command =
                                                code_content.trim_end().to_string()
                                        }
                                        "Test" => test = code_content.trim_end().to_string(),
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }
                Project {
                    dependencies: requirements,
                    solution_code: solution_py,
                    test_code: test_py,
                    install_dependency_command: Some(install_dependency_command),
                    test_command: test,
                    lang: Lang::Python,
                    ..Default::default()
                }
            }
            Lang::Scala => {
                let mut sbt = String::new();
                let mut solution_sc = String::new();
                let mut test_sc = String::new();
                let mut build = String::new();
                let mut test = String::new();

                let positions = LLMResponse::parse_positions(response);
                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();

                        if section_name.contains("build.sbt") {
                            sbt = content;
                        } else if section_name.contains("src/main/scala/Solution.scala") {
                            solution_sc = content;
                        } else if section_name.contains("src/test/scala/SolutionTest.scala") {
                            test_sc = content;
                        } else if section_name.contains("Compile") {
                            build = content;
                        } else if section_name.contains("Test") {
                            test = content;
                        }
                    }
                }

                if sbt == "" {
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
                                        "build.sbt" => sbt = code_content.trim_end().to_string(),
                                        "src/main/scala/Solution.scala" => {
                                            solution_sc = code_content.trim_end().to_string()
                                        }
                                        "src/test/scala/SolutionTest.scala" => {
                                            test_sc = code_content.trim_end().to_string()
                                        }
                                        "Compile" => build = code_content.trim_end().to_string(),
                                        "Test" => test = code_content.trim_end().to_string(),
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }
                Project {
                    dependencies: sbt,
                    solution_code: solution_sc,
                    test_code: test_sc,
                    build_command: build,
                    test_command: test,
                    lang: Lang::Scala,
                    ..Default::default()
                }
            }
            Lang::Swift => {
                let mut package = String::new();
                let mut solution_sw = String::new();
                let mut test_sw = String::new();
                let mut build = String::new();
                let mut test = String::new();

                let positions = LLMResponse::parse_positions(response);

                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();

                        if section_name.contains("Package.swift") {
                            package = content;
                        } else if section_name.contains("Sources/Solution/main.swift") {
                            solution_sw = content;
                        } else if section_name.contains("Tests/SolutionTests/SolutionTests.swift") {
                            test_sw = content;
                        } else if section_name.contains("Compile") {
                            build = content;
                        } else if section_name.contains("Test") {
                            test = content;
                        }
                    }
                }

                if package == "" {
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
                                        "Package.swift" => {
                                            package = code_content.trim_end().to_string()
                                        }
                                        "Sources/Solution/main.swift" => {
                                            solution_sw = code_content.trim_end().to_string()
                                        }
                                        "Tests/SolutionTests/SolutionTests.swift" => {
                                            test_sw = code_content.trim_end().to_string()
                                        }
                                        "Compile" => build = code_content.trim_end().to_string(),
                                        "Test" => test = code_content.trim_end().to_string(),
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }
                Project {
                    dependencies: package,
                    solution_code: solution_sw,
                    test_code: test_sw,
                    build_command: build,
                    test_command: test,
                    lang: Lang::Swift,
                    ..Default::default()
                }
            }
            Lang::TypeScript => {
                let mut package = String::new();
                let mut typescript_config = String::new();
                let mut solution = String::new();
                let mut test_code = String::new();
                let mut build = String::new();
                let mut test = String::new();

                let positions = LLMResponse::parse_positions(response);

                for i in 0..positions.len() - 1 {
                    let (section_name, start) = &positions[i];
                    let end = positions[i + 1].1;

                    let section_content = &response[*start..end];

                    // Регулярное выражение для извлечения блоков кода (с учетом многострочных текстов)
                    let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();

                    if let Some(cap) = re_code_block.captures(section_content) {
                        let content = cap.get(1).unwrap().as_str().to_string();

                        if section_name.contains("package.json") {
                            package = content;
                        } else if section_name.contains("src/solution.ts") {
                            solution = content;
                        } else if section_name.contains("tsconfig.json") {
                            typescript_config = content;
                        } else if section_name.contains("src/solution.test.ts") {
                            test_code = content;
                        } else if section_name.contains("Install") {
                            build = content;
                        } else if section_name.contains("Test") {
                            test = content;
                        }
                    }
                }

                if package == "" {
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
                                        "package.json" => {
                                            package = code_content.trim_end().to_string()
                                        }
                                        "src/solution.ts" => {
                                            solution = code_content.trim_end().to_string()
                                        }
                                        "src/solution.test.ts" => {
                                            test_code = code_content.trim_end().to_string()
                                        }
                                        "tsconfig.json" => {
                                            typescript_config = code_content.trim_end().to_string()
                                        }
                                        "Install" => build = code_content.trim_end().to_string(),
                                        "Test" => test = code_content.trim_end().to_string(),
                                        _ => (),
                                    }

                                    // Break out of the inner loop to process the next section
                                    break;
                                }
                            }
                        }
                    }
                }
                Project {
                    dependencies: package,
                    solution_code: solution,
                    test_code,
                    additional_config: vec![typescript_config],
                    build_command: build,
                    test_command: test,
                    lang: Lang::TypeScript,
                    ..Default::default()
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
