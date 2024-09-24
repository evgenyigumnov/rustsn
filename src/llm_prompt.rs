use std::collections::HashMap;

#[derive(Debug)]
pub struct Prompt {
    prompts: HashMap<String, String>,
}

impl Prompt {
    pub fn new(file_name: &str) -> Prompt {
        let mut prompts = Prompt {
            prompts: HashMap::new(),
        };
        let content = std::fs::read_to_string(file_name).unwrap();

        let mut prompt_name = String::new();
        let mut prompt_content = String::new();
        for line in content.lines() {
            if line.starts_with("[[[") {
                if !prompt_name.is_empty() {
                    prompt_content = replace_last_multiple_return_to_one(&prompt_content);
                    prompts.prompts.insert(prompt_name, prompt_content);
                }
                prompt_name = line[3..line.len() - 3].to_string();
                prompt_content = String::new();
            } else {
                prompt_content.push_str(line);
                prompt_content.push_str("\n");
            }
        }
        prompt_content = replace_last_multiple_return_to_one(&prompt_content);
        prompts.prompts.insert(prompt_name, prompt_content);

        prompts
    }

    pub fn create(&self, key: &str, params: &Vec<String>) -> String {
        let mut prompt = self.prompts.get(key).unwrap().clone();
        prompt = construct_prompt(&prompt, params);
        prompt
    }
}

fn construct_prompt(template: &str, replace: &Vec<String>) -> String {
    let mut prompt = template.to_string();
    for (i, r) in replace.iter().enumerate() {
        let placeholder = format!("{{{{{{{}}}}}}}", i); // "{{{0}}}"
        prompt = prompt.replace(&placeholder, r);
    }
    if prompt.contains("{{{") {
        panic!("Not all placeholders were replaced");
    }
    prompt
}

fn replace_last_multiple_return_to_one(input: &str) -> String {
    let trimmed = input.trim_end_matches('\n');
    let mut result = String::from(trimmed);
    if input.ends_with('\n') {
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_last_multiple_return_to_one() {
        let input = "\n123\n\nWrite on Rust language code of this function (without example of usage like main function):\n```rust\nfn solution(\n\n\n";
        let expected_output = "\n123\n\nWrite on Rust language code of this function (without example of usage like main function):\n```rust\nfn solution(\n";
        let output = replace_last_multiple_return_to_one(input);
        assert_eq!(output, expected_output);
    }
    #[test]
    fn test_prompt() {
        let content = r#"[[[generate_code_prompt_template]]]

{{{0}}}

Write on Rust language code of this function (without example of usage like main function):
```rust
fn solution(

[[[rewrite_code_prompt_template]]]

{{{0}}}
Rust language code of this function:
```rust
{{{1}}}
```
Try to compile this code:
'''bash
cargo build
'''
Result of compilation:
'''console
{{{2}}}
'''"#;
        std::fs::write("test.p", content).unwrap();
        let prompt = Prompt::new("test.p");
        println!("{:#?}", prompt);
        assert_eq!(prompt.create("generate_code_prompt_template", &vec!["123".to_string()]), "\n123\n\nWrite on Rust language code of this function (without example of usage like main function):\n```rust\nfn solution(\n");
        std::fs::remove_file("test.p").unwrap();
    }
    #[test]
    fn test_construct_prompt() {
        let template = "This is a template with {{{0}}} and {{{1}}}";
        let replace = vec!["first".to_string(), "second".to_string()];
        let expected = "This is a template with first and second";
        assert_eq!(construct_prompt(template, &replace), expected);
    }
}
