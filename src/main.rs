mod cache;
mod llm_prompt;
mod build_tool;

mod llm_api;
mod llm_parser;
mod state_machine;


const DEBUG: bool = false;
const MAX_NUMBER_OF_ATTEMPTS:i32 = 30;
fn main() {

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







