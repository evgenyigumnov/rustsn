# rustsn - This Rust-based tool generates, compiles, and tests code using LLMs, resolves dependencies, and provides explanations of existing code through embeddings.

## Features

1. **generate function** command is used to generate code snippets based on user-provided explanations.
2. TODO: **generate application** command is used to generate seed project code based on user-provided explanations.
3. **ask** command is used to get explanation by existing codes of your project based on user-provided question.

## Supported languages by feature
| language | generate function | generate application | ask |
| --- | --- |----------------------|-----|
| Rust | + | -                    | +   |
| Python | + | -                    | -   |
| JavaScript | + | -                    | -   |
| TypeScript | + | -                    | -   |
| Java | + | -                    | -   |
| Kotlin | + | -                    | -   |
| Swift | + | -                    | -   |
| PHP | + | -                    | -   |
| Scala | + | -                    | -   |


## Project name explanation

Project name "rustsn" is a combination of "Rust" and "Snippet" words. Code snippets are generated by the tool written in Rust language.


## Installation

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).
- **Make a decision**: Use Ollama (free and launched on your machine) or the OpenAI API (paid and launched on OpenAI servers).
- **If you choose Ollama**: Required for LLM interactions. Install from [Ollama's official site](https://ollama.ai/).
  - Download Ollam models  
   ```bash
   ollama pull gemma2:9b  # if your need "generate" command functionality
   ollama pull bge-large  # if your need "ask"  command functionality for existed project code
   ```
- **If you choose OpenAI API Key**: Create file "token.txt" in the root folder and put your OpenAI API key there.

### Clone the Repository

```bash
git clone https://github.com/evgenyigumnov/rustsn.git
cd rustsn
```

## Usage - Generate Function

1. **Start the Program**

   ```bash
   cargo run -- generate function --lang=rust
   ```

2. **Provide an Explanation**

   The program will prompt:

   ```
   Explain what the function should do:
   ```

   Enter a detailed explanation of the function you want to generate.
   ```
   parse json string and return struct User (age, name)
   ```
3. **Completion**

   Once the code compiles and all tests pass, the final code and tests will be displayed and result of work will be saved in `sandbox` folder.

For example:

```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn solution(json_string: &str) -> Result<User, serde_json::Error> {
    let user: User = serde_json::from_str(json_string)?;
    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let json_string = r#"{"name": "John Doe", "age": 30}"#;
        let user = solution(json_string).unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.age, 30);
    }

    #[test]
    fn test_solution_invalid_json() {
        let json_string = r#"{"name": "John Doe", "age": }"#;
        assert!(solution(json_string).is_err());
    }
}

Finished
```

## Usage - Ask

1. **Start the Program**

   ```bash
   cargo run -- --lang=rust ask /path/to/your/project
   ```

2. **Provide an Explanation**

   The program will prompt:

   ```
   Enter the question about your project sources:
   ```
   
   Enter a question about your project sources.
   ```
   How work parse function for PDF files?
   ```
3. **Completion**

   The program will return the explanation based on the existing code of your project.
```
Find closest files:
File: ../shiva/lib\src\pdf.rs
...

Answer: The `parse` function for PDF files in the provided Rust code is implemented as part of the `Transformer` struct in the `pdf.rs` file. This function is responsible for converting a PDF document into a `Document` object composed of various `Element` types. Here's a detailed breakdown of how it works:

1. **Load the PDF Document**:
   - The function takes a reference to a `Bytes` object, which contains the PDF data.
   - It uses the `lopdf` library to load the PDF document from memory using `PdfDocument::load_mem`.

2. **Iterate Through Pages**:
   - The function retrieves the pages of the PDF using `pdf_document.get_pages()`.
   - It iterates over each page to process its contents.

3. **Process Page Contents**:
   - For each page, it retrieves the contents using `pdf_document.get_page_contents(page_id)`.
   - It iterates over each content object in the page and calls the `parse_object` function to process it.

4. **Parse Individual Objects**:
   - The `parse_object` function is responsible for interpreting the contents of each object in the PDF.
   - It decodes text using the `PdfDocument::decode_text` method, manages element types like `List`, `Paragraph`, and `Text`, and handles operations associated with text positioning and font changes (e.g., "Tm", "Tf", "Tj", "TJ", "ET").

5. **Text Collection**:
   - The function `collect_text` is used to gather and decode text from PDF objects, considering encoding and operand types.
   - It adds decoded text to a string and determines when new elements like lists or paragraphs should be started based on the content.

6. **Construct Document Elements**:
   - The function constructs `Element` types such as `Text`, `Paragraph`, and `List`, and adds them to a vector of elements.
   - These elements are used to build the final `Document` object, representing the structure and content of the PDF.

7. **Return the Document**:
   - After processing all pages and objects, the function returns a `Document` instance containing all the parsed elements.

In summary, the `parse` function for PDF files reads the PDF data, iterates through its pages and content objects, decodes text, and constructs a structured `Document` composed of various elements, which can then be used for further processing or transformation.
```


## Contributing

I would love to see contributions from the community. If you experience bugs, feel free to open an issue. If you would like to implement a new feature or bug fix, please follow the steps:
1. Do fork 
2. Add comment to the [issue](https://github.com/evgenyigumnov/rustsn/issues) that you are going to work on it
3. Create pull request

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rustsn by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>


## Versions
### 0.17.0 - "Ask" command 11 October 2024
- Add "ask" command to get explanation by existing codes of your project based on user-provided question

### 0.16.0 - MIT or Apache-2.0 26 September 2024
- Add MIT or Apache-2.0 license

### 0.15.0 - TypeScript 24 September 2024
- Add TypeScript language support

### 0.14.0 - Swift 23 September 2024
- Add Swift language support

### 0.13.0 - Kotlin 23 September 2024
- Add Kotlin language support

### 0.12.0 - Python 22 September 2024
- Add Python language support

### 0.11.0 - PHP 22 September 2024
- Add PHP language support
 
### 0.10.0 - JavaScript 22 September 2024
- Add JavaScript language support

### 0.9.0 - Scala 22 September 2024
- Add Scala language support

### 0.8.0 - Java 22 September 2024
- Add Java language support

### 0.7.0 - Simplification 22 September 2024
- Simplify state machine logic and remove logic.md file
- Simplify prompt.txt file

### 0.6.0 - Add "clap" crate 21 September 2024
- Add --lang parameter to specify the language of the generated code (Rust, Python, C, JavaScript, Java, TypeScript, CPP, Scala, Kotlin, Swift)

### 0.5.0 - Support multi-language code generation 21 September 2024
- Make decision to support multi-language code generation: Python, C, JavaScript, Java, TypeScript, CPP, Scala, Kotlin, Swift

### 0.4.0 - LLM Generate Result Extraction - 20 September 2024
- Extract_code function replaced by extract_code, extract_dep, extract_test functions

### 0.3.0 - State Machine - 20 September 2024
- Support OpenAI API

### 0.2.0 - State Machine - 19 September 2024
- Moved prompts from code to "rust.prompt" file
- Moved logic from code to "logic.md" file

### 0.1.0 - Prototype - 17 September 2024
- Code Generation
- Automated Compilation
- Dependency Resolution
- Test Generation
- Error Correction
- Caching Mechanism 

