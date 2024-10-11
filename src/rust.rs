mod tests {
    #[test]
    fn test_parse_llm_response() {
        for i in 1..=9 {
            let file = format!("./test_data/rust_create_{}.txt", i);
            let response = std::fs::read_to_string(file).unwrap();
            let mut project =
                crate::llm_response::LLMResponse::parse_llm_response(&response, crate::Lang::Rust);
            project.build_command = crate::utils::remove_comments(&project.build_command);
            project.test_command = crate::utils::remove_comments(&project.test_command);

            println!("{:#?}", project);
            assert!(!project.dependencies.is_empty());
            assert!(!project.solution_code.is_empty());
            assert!(!project.build_command.is_empty());
            assert!(!project.test_command.is_empty());
        }
    }
}
