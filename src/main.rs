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
        Lang::CSharp => println!("Selected language: C#"),
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
                    handle_ask_command(
                        path,
                        &lang,
                        &llm,
                        &mut cache,
                        &prompt,
                        vec![String::from("txt"), String::from("toml")],
                        vec![String::from("target")],
                        "Explain how this code works and what it do:",
                        "Use functions from code above to give answer for this question:",
                    );
                }
                Lang::CSharp => {
                    handle_ask_command(
                        path,
                        &lang,
                        &llm,
                        &mut cache,
                        &prompt,
                        vec![String::from("cs")],
                        vec![String::from("bin"), String::from("obj")],
                        "Explain how this code works and what it does:",
                        "Use the code above to answer the following question:",
                    );
                }
                Lang::JavaScript => {
                    handle_ask_command(
                        path,
                        &lang,
                        &llm,
                        &mut cache,
                        &prompt,
                        vec![String::from("js")],
                        vec![String::from("node_modules")],
                        "Explain how this JavaScript code works and what it does:",
                        "Use the code above to answer the following question:",
                    );
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

fn handle_ask_command(
    path: &String,
    _lang: &Lang,
    llm: &llm_api::LLMApi,
    cache: &mut cache::Cache,
    prompt: &llm_prompt::Prompt,
    extensions: Vec<String>,
    exclude_dirs: Vec<String>,
    _explain_prompt: &str,
    answer_prompt: &str,
) {
    let files = file_explorer::explore_files(&path, &extensions, &exclude_dirs);
    let mut vectors: HashMap<String, Vec<f32>> = HashMap::new();
    for file in &files {
        println!("File: {:?}", file);
        let content_file = std::fs::read_to_string(file).unwrap();
        let content = format!("# {}\r\n{}", file, content_file);

        // let prompt_template = format!("{}\r\n{}", content, _explain_prompt);
        // let llm_code_explanation = llm.request(&prompt_template, &Vec::new(), cache, prompt);
        // let full_content = format!("{}\r\n{}", content, llm_code_explanation);
        // let emb = llm.emb(&content, cache, &full_content);
        let emb = llm.emb(&content, cache, &content);
        vectors.insert(file.clone(), emb);
    }
    let mut target_vectors: HashMap<String, Vec<f32>> = HashMap::new();

    let medical = "The patient, a 45-year-old male, presents with a three-month history of intermittent chest pain, primarily occurring during physical exertion. The pain is described as a pressure-like sensation, radiating to the left arm and jaw, and is relieved with rest. The patient has a history of hypertension and is currently on medication for high blood pressure. No prior history of heart disease or recent infections is reported. He denies any shortness of breath, palpitations, or dizziness. A family history of cardiovascular disease is noted, with both parents having a history of myocardial infarction. The patient is a non-smoker and denies alcohol or drug use.";
    let invoice = "Please be advised that the total amount due for the services rendered on [insert date] is [insert amount]. This includes [brief description of services or products provided]. Kindly remit payment by [insert due date] to the following account details: [insert payment details]. Should you have any questions regarding this invoice, feel free to contact us at [insert contact information]. Thank you for your prompt attention to this matter.";
    let will = "I, [Full Name], of [City, State], being of sound mind and body, do hereby declare this to be my last will and testament. I revoke any and all previous wills and codicils made by me. I direct that all my just debts, funeral expenses, and the expenses of my last illness be paid as soon as practicable after my death. I leave all my personal property, including but not limited to [list specific items if desired], to my [relationship, e.g., spouse, children], [Full Name(s)], in equal shares. Should any of my beneficiaries predecease me, their share shall be distributed to their heirs or next of kin, as provided by law.";
    let diary = "Today was a mix of everything. I woke up feeling excited, but by lunchtime, everything started to annoy me. School was the same, but my friends made it better. We talked about random stuff and laughed a lot, which helped me forget about all the little things that were bothering me. I spent most of the afternoon in my room, listening to music and thinking about life. Sometimes, it feels like no one really understands what’s going on in my head, but that’s okay. I guess it’s just part of growing up.";
    let license = "This software is provided 'as-is', without any express or implied warranties, including but not limited to the implied warranties of merchantability and fitness for a particular purpose. In no event shall the authors or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the software. Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the 'Software'), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, subject to the following conditions...";
    let srs = "To develop an Android application, several key requirements must be considered. First, a solid understanding of the Android SDK and Java or Kotlin programming languages is essential, as these are the primary tools for building Android apps. The application must be compatible with a wide range of Android devices, ensuring it adapts to various screen sizes and resolutions. It is also crucial to follow Android's material design guidelines for the user interface to create a consistent and intuitive user experience. The app should be optimized for performance, including efficient memory management and responsiveness, while handling device-specific constraints such as battery life and network conditions. Additionally, ensuring proper integration with Android services like notifications, background processing, and storage is critical for delivering a seamless experience. Lastly, the app must comply with Android’s security standards, protecting user data and ensuring secure communication with any external servers or APIs.";
    let v1 = llm.emb(medical, cache, medical);
    let v2 = llm.emb(invoice, cache, invoice);
    let v3 = llm.emb(will, cache, will);
    let v4 = llm.emb(diary, cache, diary);
    let v5 = llm.emb(license, cache, license);
    let v6 = llm.emb(license, cache, srs);
    // println!("Medical: {:?}", v1);
    // println!("Invoice: {:?}", v2);
    // println!("Other: {:?}", v3);
    target_vectors.insert("Medical".to_string(), v1);
    target_vectors.insert("Invoice".to_string(), v2);
    target_vectors.insert("Will".to_string(), v3);
    target_vectors.insert("Diary".to_string(), v4);
    target_vectors.insert("License".to_string(), v5);
    target_vectors.insert("SRS".to_string(), v6);


    for vector in vectors {
        let result = vector_utils::find_closest(&vector.1, &target_vectors);
        let limited_result = result.iter().take(1).collect::<Vec<_>>();
        for r in &limited_result {
            if (r.1 < 17.0) {
                println!("{} - {}: {}",vector.0, r.0, r.1);
            } else {
                println!("{} - {}: {}",vector.0, "Other", r.1);
            }
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
            Lang::CSharp => write!(f, "cs"),
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
