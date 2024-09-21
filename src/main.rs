use clap::{Arg, Command};
use std::str::FromStr;

mod cache;
mod llm_prompt;
mod build_tool;

mod llm_api;
mod llm_parser;
mod state_machine;


const DEBUG: bool = false;
const MAX_NUMBER_OF_ATTEMPTS:i32 = 30;
fn main() {
    let matches = Command::new("rustsn - Rust Snippets")
        .version("0.6.0")
        .author("Evgeny Igumnov <igumnovnsk@gmail.com>")
        .about("Generation, compilation, and testing of code using LLMs")
        .arg(
            Arg::new("lang")
                .long("lang")
                .value_name("LANG")
                .help("Sets the programming language")
                .default_value("RUST")
                .value_names(&["RUST", "JAVA", "SCALA", "PYTHON", "C", "CPP", "KOTLIN", "SWIFT"]),
        )
        .get_matches();

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
        _ => {println!("Unsupported language: {:?}", lang); std::process::exit(1);}
    }

    let states_str = std::fs::read_to_string("logic.md").unwrap();
    let mut cache = cache::Cache::new();
    let prompt = llm_prompt::Prompt::new("prompt.txt");
    // if file token.txt exists
    let llm= if std::path::Path::new("token.txt").exists() {
        println!("Use OpenAI API");
        println!("");
        let token = std::fs::read_to_string("token.txt").unwrap();
        llm_api::LLMApi::new(llm_api::ModelType::OpenAI {
            api_key: token.trim().to_string()
        })
    }
    else {
        println!("Use Ollama API");
        println!("");
        llm_api::LLMApi::new(llm_api::ModelType::Ollama)
    };

    println!("Use '\\' char in the end of line for multiline mode or just copy-paste multiline text.");
    println!("");
    println!("For launch code generation, type ENTER twice after the last line of the prompt.");
    println!("");
    println!("Explain what the function should do:");
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

        let now_sec = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();

        if start_sec == 0  {
            start_sec = now_sec;
        } else {
            if now_sec - start_sec < 100 {
                continue;
            } else {
                if line.ends_with("\\\r\n") {
                    continue;
                }
                break
            }
        }
    }
    question = lines.join("");
    question = question.trim().to_string();
    question.push('\r');
    question.push('\n');


    let mut code = "".to_string();
    let mut dependencies = "".to_string();
    let mut tests = "".to_string();
    let mut output = "".to_string();

    println!("====================");
    state_machine::run_state_machine(&states_str, &question, &mut code, &mut dependencies, &mut tests, &mut output, &prompt, &mut cache, &llm);
    println!("++++++++ Finished ++++++++++++");
    println!("\n{}\n{}\n{}", code, dependencies, tests);
    println!("++++++++ Finished ++++++++++++");


}


#[derive(Debug, Clone)]
enum Lang {
    Rust,
    Java,
    Scala,
    Python,
    C,
    Cpp,
    Kotlin,
    Swift,
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Lang::Rust),
            "java" => Ok(Lang::Java),
            "scala" => Ok(Lang::Scala),
            "python" => Ok(Lang::Python),
            "c" => Ok(Lang::C),
            "cpp" => Ok(Lang::Cpp),
            "kotlin" => Ok(Lang::Kotlin),
            "swift" => Ok(Lang::Swift),
            _ => Err(format!("Unsupported language: {}", s)),
        }
    }
}





