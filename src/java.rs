mod tests {
    #[test]
    fn test_parse_llm_response() {
        for i in 1..=5 {
            let file = format!("./test_data/java_create_{}.txt", i);
            let response = std::fs::read_to_string(file).unwrap();
            let project = crate::llm_response::LLMResponse::parse_llm_response(&response, crate::Lang::Java);

            println!("{:#?}", project);
            assert!(!project.dependencies.is_empty());
            assert!(!project.solution_code.is_empty());
            assert!(!project.test_code.is_empty());
            assert!(!project.build_command.is_empty());
            assert!(!project.test_command.is_empty());
        }
    }
}
