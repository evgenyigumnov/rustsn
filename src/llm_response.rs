use regex::Regex;
use std::collections::HashMap;

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

    fn extract_sections(response: &str, section_names: &[&str]) -> HashMap<String, String> {
        let mut sections_content = HashMap::new();
        let positions = LLMResponse::parse_positions(response);

        // First method: use positions and extract content
        for i in 0..positions.len() - 1 {
            let (section_name, start) = &positions[i];
            let end = positions[i + 1].1;

            let section_content = &response[*start..end];

            // Regular expression to extract code blocks (including multiline texts)
            let re_code_block = Regex::new(r"(?s)```.*?\n(.*?)```").unwrap();
            if let Some(cap) = re_code_block.captures(section_content) {
                let content = cap.get(1).unwrap().as_str().to_string();
                for &desired_section in section_names {
                    if section_name.contains(desired_section) {
                        sections_content.insert(desired_section.to_string(), content.clone());
                    }
                }
            }
        }

        // Check if all desired sections have been found
        let missing_sections: Vec<_> = section_names
            .iter()
            .filter(|&&s| !sections_content.contains_key(s))
            .collect();

        // If some sections are missing, try alternative method
        if !missing_sections.is_empty() {
            let mut lines = response.lines().peekable();
            let re = Regex::new(r"- \*\*(.*)\*\*").unwrap();
            while let Some(line) = lines.next() {
                if line.starts_with("## ") || line.starts_with("### ") || re.is_match(line) {
                    let section_title = line
                        .trim_start_matches(&['#', ' '][..])
                        .trim()
                        .replace("`", "")
                        .replace("- **", "")
                        .replace("*", "")
                        .replace(":", "");

                    // Now check if section_title matches any desired section
                    for &desired_section in missing_sections.iter() {
                        if section_title.contains(desired_section) {
                            // Now extract code block
                            while let Some(line) = lines.next() {
                                if line.trim().starts_with("```") {
                                    // Capture the code block content
                                    let mut code_content = String::new();
                                    while let Some(line) = lines.next() {
                                        if line.trim().starts_with("```") {
                                            // End of code block
                                            break;
                                        } else {
                                            code_content.push_str(line);
                                            code_content.push('\n');
                                        }
                                    }
                                    sections_content.insert(
                                        desired_section.to_string(),
                                        code_content.trim_end().to_string(),
                                    );
                                    break;
                                }
                            }
                            break; // break out of for loop over desired sections
                        }
                    }
                }
            }
        }

        sections_content
    }

    pub fn parse_llm_response(response: &str, language: Lang) -> Project {
        match language {
            Lang::Rust => {
                let desired_sections = ["Cargo.toml", "src/lib.rs", "Build", "Test"];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let cargo_toml = sections_content
                    .get("Cargo.toml")
                    .cloned()
                    .unwrap_or_default();
                let lib_rs = sections_content
                    .get("src/lib.rs")
                    .cloned()
                    .unwrap_or_default();
                let build = sections_content.get("Build").cloned().unwrap_or_default();
                let test = sections_content.get("Test").cloned().unwrap_or_default();

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
                let desired_sections = [
                    "package.json",
                    "src/solution.js",
                    "Install",
                    "Test",
                    "src/solution.test.js",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let pkg_json = sections_content
                    .get("package.json")
                    .cloned()
                    .unwrap_or_default();
                let solution = sections_content
                    .get("src/solution.js")
                    .cloned()
                    .unwrap_or_default();
                let install_dependency_command =
                    sections_content.get("Install").cloned().unwrap_or_default();
                let test = sections_content.get("Test").cloned().unwrap_or_default();
                let test_js = sections_content
                    .get("src/solution.test.js")
                    .cloned()
                    .unwrap_or_default();

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
                let desired_sections = [
                    "pom.xml",
                    "src/main/java/com/example/solution/Solution.java",
                    "src/test/java/com/example/solution/SolutionTest.java",
                    "Compile",
                    "Test",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let pom_xml = sections_content.get("pom.xml").cloned().unwrap_or_default();
                let solution_java = sections_content
                    .get("src/main/java/com/example/solution/Solution.java")
                    .cloned()
                    .unwrap_or_default();
                let test_java = sections_content
                    .get("src/test/java/com/example/solution/SolutionTest.java")
                    .cloned()
                    .unwrap_or_default();
                let build_command = sections_content.get("Compile").cloned().unwrap_or_default();
                let test_command = sections_content.get("Test").cloned().unwrap_or_default();

                Project {
                    dependencies: pom_xml,
                    solution_code: solution_java,
                    test_code: test_java,
                    build_command,
                    test_command,
                    lang: Lang::Java,
                    ..Default::default()
                }
            }
            Lang::Kotlin => {
                let desired_sections = [
                    "build.gradle",
                    "src/main/kotlin/Solution.kt",
                    "src/test/kotlin/SolutionTest.kt",
                    "Compile",
                    "Test",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let gradle = sections_content
                    .get("build.gradle")
                    .cloned()
                    .unwrap_or_default();
                let solution_kotlin = sections_content
                    .get("src/main/kotlin/Solution.kt")
                    .cloned()
                    .unwrap_or_default();
                let test_kotlin = sections_content
                    .get("src/test/kotlin/SolutionTest.kt")
                    .cloned()
                    .unwrap_or_default();
                let build_command = sections_content.get("Compile").cloned().unwrap_or_default();
                let test_command = sections_content.get("Test").cloned().unwrap_or_default();

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
                let desired_sections = [
                    "composer.json",
                    "src/Solution.php",
                    "tests/SolutionTest.php",
                    "Install",
                    "Test",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let composer = sections_content
                    .get("composer.json")
                    .cloned()
                    .unwrap_or_default();
                let solution_php = sections_content
                    .get("src/Solution.php")
                    .cloned()
                    .unwrap_or_default();
                let test_php = sections_content
                    .get("tests/SolutionTest.php")
                    .cloned()
                    .unwrap_or_default();
                let install_dependency_command =
                    sections_content.get("Install").cloned().unwrap_or_default();
                let test_command = sections_content.get("Test").cloned().unwrap_or_default();

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
                let desired_sections = [
                    "requirements.txt",
                    "solution.py",
                    "test.py",
                    "Dependencies",
                    "Test",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let requirements = sections_content
                    .get("requirements.txt")
                    .cloned()
                    .unwrap_or_default();
                let solution_py = sections_content
                    .get("solution.py")
                    .cloned()
                    .unwrap_or_default();
                let test_py = sections_content.get("test.py").cloned().unwrap_or_default();
                let install_dependency_command = sections_content
                    .get("Dependencies")
                    .cloned()
                    .unwrap_or_default();
                let test_command = sections_content.get("Test").cloned().unwrap_or_default();

                Project {
                    dependencies: requirements,
                    solution_code: solution_py,
                    test_code: test_py,
                    install_dependency_command: Some(install_dependency_command),
                    test_command,
                    lang: Lang::Python,
                    ..Default::default()
                }
            }
            Lang::Scala => {
                let desired_sections = [
                    "build.sbt",
                    "src/main/scala/Solution.scala",
                    "src/test/scala/SolutionTest.scala",
                    "Compile",
                    "Test",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let sbt = sections_content
                    .get("build.sbt")
                    .cloned()
                    .unwrap_or_default();
                let solution_sc = sections_content
                    .get("src/main/scala/Solution.scala")
                    .cloned()
                    .unwrap_or_default();
                let test_sc = sections_content
                    .get("src/test/scala/SolutionTest.scala")
                    .cloned()
                    .unwrap_or_default();
                let build = sections_content.get("Compile").cloned().unwrap_or_default();
                let test = sections_content.get("Test").cloned().unwrap_or_default();

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
                let desired_sections = [
                    "Package.swift",
                    "Sources/Solution/main.swift",
                    "Tests/SolutionTests/SolutionTests.swift",
                    "Compile",
                    "Test",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let package = sections_content
                    .get("Package.swift")
                    .cloned()
                    .unwrap_or_default();
                let solution_sw = sections_content
                    .get("Sources/Solution/main.swift")
                    .cloned()
                    .unwrap_or_default();
                let test_sw = sections_content
                    .get("Tests/SolutionTests/SolutionTests.swift")
                    .cloned()
                    .unwrap_or_default();
                let build = sections_content.get("Compile").cloned().unwrap_or_default();
                let test = sections_content.get("Test").cloned().unwrap_or_default();

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
                let desired_sections = [
                    "package.json",
                    "tsconfig.json",
                    "src/solution.ts",
                    "src/solution.test.ts",
                    "Install",
                    "Test",
                ];

                let sections_content = LLMResponse::extract_sections(response, &desired_sections);

                let package = sections_content
                    .get("package.json")
                    .cloned()
                    .unwrap_or_default();
                let typescript_config = sections_content
                    .get("tsconfig.json")
                    .cloned()
                    .unwrap_or_default();
                let solution = sections_content
                    .get("src/solution.ts")
                    .cloned()
                    .unwrap_or_default();
                let test_code = sections_content
                    .get("src/solution.test.ts")
                    .cloned()
                    .unwrap_or_default();
                let build = sections_content.get("Install").cloned().unwrap_or_default();
                let test = sections_content.get("Test").cloned().unwrap_or_default();

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
