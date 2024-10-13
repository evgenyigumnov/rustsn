use clap::{Arg, ArgAction, Command};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Mutex;

mod build_tool;
mod cache;
mod file_explorer;
mod llm_api;
mod llm_prompt;
mod llm_response;
mod state_machine;
mod utils;
mod vector_utils;

static VERBOSE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

const MAX_NUMBER_OF_ATTEMPTS: i32 = 5;
const OLLAMA_API: &str = "http://127.0.0.1:11434/api/generate";
const OLLAMA_EMB: &str = "http://127.0.0.1:11434/api/embeddings";

fn main() {
    std::env::set_var("OLLAMA_NUM_PARALLEL", "2");
    let matches = Command::new("rustsn - Rust Snippets Generator")
        .version("0.7.0")
        .author("Evgeny Igumnov <igumnovnsk@gmail.com>")
        .about("Code snippets generator via LLMs and compiler/tester via build tools")
        .help_template(
            "{bin} {version}
{about}

Usage:
    {usage}

{all-args}
",
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Enable verbose mode")
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .value_name("LANG")
                .help("Sets the programming language")
                .default_value("rust")
                .global(true)
                .value_parser(*&[
                    "rust",
                    "java",
                    "javascript",
                    "typescript",
                    "scala",
                    "kotlin",
                    "swift",
                    "php",
                    "python",
                    "cs",
                ]),
        )
        .arg(
            Arg::new("ollmod")
                .long("ollmod")
                .value_name("OLLAMA-MODEL")
                .help("Set desired ollama model")
                .default_value("qwen2.5-coder:7b")
                .global(true),
        )
        .arg(
            Arg::new("ollemb")
                .long("ollemb")
                .value_name("OLLAMA-EMBEDIDING")
                .help("Set desired ollama embedding")
                .default_value("bge-large")
                .global(true),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate code")
                .alias("g")
                .arg(
                    Arg::new("type")
                        .help("Type of generation")
                        .value_parser(*&["function", "application"])
                        .default_value("function")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("ask")
                .about("Ask a question about code in a folder")
                .alias("a")
                .arg(
                    Arg::new("path")
                        .help("Path to the source code folder")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let verbose = matches.get_one::<bool>("verbose").unwrap();
    *VERBOSE.lock().unwrap() = *verbose;

    let lang: Lang = matches
        .get_one::<String>("lang")
        .unwrap()
        .parse()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

    // Optionally, handle the selected language
    match lang {
        Lang::Rust => println!("Selected language: Rust"),
        Lang::Java => println!("Selected language: Java"),
        Lang::Scala => println!("Selected language: Scala"),
        Lang::JavaScript => println!("Selected language: JavaScript"),
        Lang::Php => println!("Selected language: PHP"),
        Lang::Python => println!("Selected language: Python"),
        Lang::Kotlin => println!("Selected language: Kotlin"),
        Lang::Swift => println!("Selected language: Swift"),
        Lang::TypeScript => println!("Selected language: TypeScript"),
        _ => {
            println!("Unimplemented language: {:?}", lang);
            std::process::exit(1);
        }
    }

    let mut cache = cache::Cache::new();

    let prompt_file_path = format!("prompt/{}.txt", lang);
    if !std::path::Path::new(&prompt_file_path).exists() {
        println!(
            "Warning: Cant find \"{}\". Downloading it from https://github.com/evgenyigumnov/rustsn/raw/HEAD/{}",
            prompt_file_path, prompt_file_path
        );

        let url = format!(
            "https://github.com/evgenyigumnov/rustsn/raw/HEAD/{}",
            prompt_file_path
        );
        match reqwest::blocking::get(&url) {
            Ok(response) => {
                if response.status().is_success() {
                    let content = response.text().unwrap();
                    // Create directories if they don't exist
                    if let Some(parent) = std::path::Path::new(&prompt_file_path).parent() {
                        std::fs::create_dir_all(parent).unwrap();
                    }
                    // Write the content to the prompt file
                    std::fs::write(&prompt_file_path, content).unwrap();
                } else {
                    eprintln!(
                        "Failed to download the prompt file: HTTP {}",
                        response.status()
                    );
                    std::process::exit(1);
                }
            }
            Err(err) => {
                eprintln!("Error downloading the prompt file: {}", err);
                std::process::exit(1);
            }
        }
    }

    let prompt = llm_prompt::Prompt::new(format!("prompt/{}.txt", lang).as_str());
    // if file token.txt exists
    let llm = if std::path::Path::new("token.txt").exists() {
        println!("Use OpenAI API");
        println!("");
        let token = std::fs::read_to_string("token.txt").unwrap();
        llm_api::LLMApi::new(llm_api::ModelType::OpenAI {
            api_key: token.trim().to_string(),
        })
    } else {
        let ollama_model: String = matches
            .get_one::<String>("ollmod")
            .unwrap()
            .parse()
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            });
        println!("Warning: Cant find \"token.txt\" file for OpenAI API integration.");
        println!("Use Ollama API: {}", OLLAMA_API);
        println!("Use Ollama model: {}", ollama_model);
        println!("");

        let emb: String = matches
            .get_one::<String>("ollemb")
            .unwrap()
            .parse()
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            });
        llm_api::LLMApi::new(llm_api::ModelType::Ollama {
            model: ollama_model,
            emb,
        })
    };

    println!(
        "Use '\\' char in the end of line for multiline mode or just copy-paste multiline text."
    );
    println!("");

    println!("For launch work with AI, type ENTER twice after the last line of the prompt.");
    println!("");

    let command = matches.subcommand_name();
    match command {
        Some("generate") => {
            println!("Explain what the function should do:");
            let question: String = ask();

            state_machine::run_state_machine(&lang, &question, &prompt, &mut cache, &llm);
            println!("++++++++ Finished ++++++++++++");
        }
        Some("ask") => {
            let path: &String = matches
                .subcommand_matches("ask")
                .unwrap()
                .get_one("path")
                .unwrap();
            println!("Path: {:?}", path);
            match lang {
                Lang::Rust => {
                    let files = file_explorer::explore_files(
                        &path,
                        &vec![String::from("rs"), String::from("toml")],
                        &vec![String::from("target")],
                    );
                    let mut vectors: HashMap<String, Vec<f32>> = HashMap::new();
                    for file in &files {
                        println!("File: {:?}", file);
                        let content_file = std::fs::read_to_string(file).unwrap();
                        let content = format!("== {} ==\r\n{}", file, content_file);

                        let prompt_template = format!(
                            "{}\r\n{}",
                            content, "Explain how this code works and what it do:"
                        );
                        let llm_question =
                            llm.request(&prompt_template, &Vec::new(), &mut cache, &prompt);

                        let emb = llm.emb(&content, &mut cache, &llm_question);
                        // println!("{:#?}", emb);
                        vectors.insert(file.clone(), emb);
                    }

                    println!("Enter the question about your project sources:");
                    let question: String = ask();
                    let target_emb = llm.emb(&question, &mut cache, &question);
                    let result = vector_utils::find_closest(&target_emb, &vectors);
                    let limited_result = result.iter().take(3).collect::<Vec<_>>();
                    println!("Find closest files:");
                    for (k, _v) in &limited_result {
                        println!("File: {}", k);
                    }
                    let files_content_vec = limited_result
                        .iter()
                        .map(|(k, _)| {
                            let content = std::fs::read_to_string(k).unwrap();
                            format!("== {} ==\r\n{}", k, content)
                        })
                        .collect::<Vec<_>>();
                    let files_content = files_content_vec.join("\r\n");

                    let prompt_template = format!(
                        "{}\r\n{}\r\n{}",
                        files_content,
                        "Use functions from code above to give answer for this question: ",
                        question
                    );
                    if *VERBOSE.lock().unwrap() {
                        println!("Request: {}", prompt_template);
                    }
                    let answer = llm.request(&prompt_template, &Vec::new(), &mut cache, &prompt);

                    println!("++++++++ Answer ++++++++++++");

                    println!("Answer: {}", answer);
                }
                Lang::CSharp => {
                    let files = file_explorer::explore_files(
                        &path,
                        &vec![String::from("cs")], // C# files have .cs extension
                        &vec![String::from("bin"), String::from("obj")], // Exclude build directories
                    );
                    let mut vectors: HashMap<String, Vec<f32>> = HashMap::new();
                    for file in &files {
                        println!("File: {:?}", file);
                        let content_file = std::fs::read_to_string(file).unwrap();
                        let content = format!("== {} ==\r\n{}", file, content_file);

                        let prompt_template = format!(
                            "{}\r\n{}",
                            content, "Explain how this code works and what it does:"
                        );
                        let llm_question =
                            llm.request(&prompt_template, &Vec::new(), &mut cache, &prompt);

                        let emb = llm.emb(&content, &mut cache, &llm_question);
                        vectors.insert(file.clone(), emb);
                    }

                    println!("Enter the question about your project sources:");
                    let question: String = ask();
                    let target_emb = llm.emb(&question, &mut cache, &question);
                    let result = vector_utils::find_closest(&target_emb, &vectors);
                    let limited_result = result.iter().take(3).collect::<Vec<_>>();
                    println!("Find closest files:");
                    for (k, _v) in &limited_result {
                        println!("File: {}", k);
                    }
                    let files_content_vec = limited_result
                        .iter()
                        .map(|(k, _)| {
                            let content = std::fs::read_to_string(k).unwrap();
                            format!("== {} ==\r\n{}", k, content)
                        })
                        .collect::<Vec<_>>();
                    let files_content = files_content_vec.join("\r\n");

                    let prompt_template = format!(
                        "{}\r\n{}\r\n{}",
                        files_content,
                        "Use the code above to answer the following question: ",
                        question
                    );
                    if *VERBOSE.lock().unwrap() {
                        println!("Request: {}", prompt_template);
                    }
                    let answer = llm.request(&prompt_template, &Vec::new(), &mut cache, &prompt);

                    println!("++++++++ Answer ++++++++++++");

                    println!("Answer: {}", answer);
                }

                _ => {
                    println!("Unsupported language: {:?}", lang);
                    std::process::exit(1);
                }
            }

            println!("++++++++ Finished ++++++++++++");
        }
        _ => {
            println!("Unknown command, please use 'generate' or 'ask'");
            std::process::exit(1);
        }
    }
}

fn ask() -> String {
    let mut question;
    let mut lines = vec![];
    let mut start_sec = 0 as u128;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line.ends_with("\\\r\n") {
            let mut line_clone = line.clone();
            line_clone.pop();
            line_clone.pop();
            line_clone.pop();
            line_clone.push('\r');
            line_clone.push('\n');
            lines.push(line_clone.clone());
        } else {
            lines.push(line.clone());
        }

        let now_sec = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        if start_sec == 0 {
            start_sec = now_sec;
        } else {
            if now_sec - start_sec < 100 {
                continue;
            } else {
                if line.ends_with("\\\r\n") {
                    continue;
                }
                break;
            }
        }
    }
    question = lines.join("");
    question = question.trim().to_string();
    question.push('\r');
    question.push('\n');
    question
}

#[derive(Debug, Clone)]
enum Lang {
    Rust,
    Java,
    JavaScript,
    TypeScript,
    Scala,
    Python,
    C,
    Cpp,
    Kotlin,
    Php,
    Swift,
    CSharp,
    Unknown,
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::Rust => write!(f, "rust"),
            Lang::Java => write!(f, "java"),
            Lang::JavaScript => write!(f, "javascript"),
            Lang::TypeScript => write!(f, "typescript"),
            Lang::Scala => write!(f, "scala"),
            Lang::Python => write!(f, "python"),
            Lang::C => write!(f, "c"),
            Lang::Cpp => write!(f, "cpp"),
            Lang::Kotlin => write!(f, "kotlin"),
            Lang::Php => write!(f, "php"),
            Lang::Swift => write!(f, "swift"),
            _ => {
                return Ok(());
            }
        }
    }
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Lang::Rust),
            "java" => Ok(Lang::Java),
            "javascript" => Ok(Lang::JavaScript),
            "typescript" => Ok(Lang::TypeScript),
            "scala" => Ok(Lang::Scala),
            "python" => Ok(Lang::Python),
            "c" => Ok(Lang::C),
            "cpp" => Ok(Lang::Cpp),
            "kotlin" => Ok(Lang::Kotlin),
            "php" => Ok(Lang::Php),
            "cs" => Ok(Lang::CSharp),
            "swift" => Ok(Lang::Swift),
            _ => Err(format!("Unsupported language: {}", s)),
        }
    }
}
