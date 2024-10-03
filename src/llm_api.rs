use crate::cache::Cache;
use crate::llm_prompt::Prompt;
use crate::{OLLAMA_API, OLLAMA_EMB};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// const STOP_WORDS: &[&str] = &[
//     "**Explanation",
//     "**Notes",
//     "### Explanation",
//     "**Additional Notes",
// ];
const STOP_WORDS: &[&str] = &[];
const MAX_TOKENS: i32 = 1000;
pub struct LLMApi {
    model_type: ModelType,
}

#[derive(Debug, PartialEq)]
pub enum ModelType {
    Ollama {
        model: String,
        emb: String,
    },
    OpenAI { api_key: String },
}

impl LLMApi {
    pub fn new(model_type: ModelType) -> LLMApi {
        LLMApi { model_type }
    }

    pub fn request(
        &self,
        prompt_template: &str,
        params: &Vec<String>,
        cache: &mut Cache,
        prompt: &Prompt,
    ) -> String {
        match &self.model_type {
            ModelType::Ollama { model, .. } => {
                let prompt = prompt.create(prompt_template, params);
                let stop = STOP_WORDS;
                let request = OllamaRequest {
                    // model: "qwen2.5-coder:7b".to_string(), // smart model but slow
                    // model: "qwen2.5-coder:1.5b".to_string(), // smart model but slow
                    model: model.to_string(),
                    // model: "gemma2:2b".to_string(), // fast but very stupid model - excellent for fast testing
                    //  model: "gemma2".to_string(), // medium model
                    prompt: prompt.to_string(),
                    stream: false,
                    options: OllamaOptions {
                        num_predict: MAX_TOKENS,
                        stop: stop.iter().map(|s| s.to_string()).collect(),
                    },
                };

                let request_str = serde_json::to_string(&request).unwrap();
                println!("Request: {}", request.prompt);
                println!("===============");

                let response_opt = cache.get(&request_str);
                let response = match response_opt {
                    None => {
                        let client = Client::builder()
                            .timeout(Duration::from_secs(60 * 10))
                            .build()
                            .unwrap();
                        println!("Request in progress");

                        let response = client
                            .post(OLLAMA_API)
                            .json(&request)
                            .send()
                            .unwrap()
                            .json::<OllamaResponse>()
                            .unwrap();
                        cache.set(request_str.clone(), response.response.clone());
                        response.response
                    }
                    Some(result) => result.to_string(),
                };

                println!("Response: {}", response);
                response
            }
            ModelType::OpenAI { api_key } => {
                let user_prompt = prompt.create(prompt_template, params);
                let messages = vec![ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                }];

                let request = OpenAIChatRequest {
                    model: "gpt-4o-2024-08-06".to_string(),
                    messages,
                    max_tokens: MAX_TOKENS,
                    temperature: 0.7,
                    stop: Some(STOP_WORDS.iter().map(|s| s.to_string()).collect()),
                };

                let request_str = serde_json::to_string(&request).unwrap();
                println!("OpenAI Chat Request: {}", user_prompt);
                println!("===============");

                let response_opt = cache.get(&request_str);
                let response = match response_opt {
                    None => {
                        let client = Client::builder()
                            .timeout(Duration::from_secs(60 * 5))
                            .build()
                            .unwrap();

                        let response = client
                            .post("https://api.openai.com/v1/chat/completions")
                            .bearer_auth(api_key)
                            .json(&request)
                            .send()
                            .unwrap()
                            .json::<OpenAIChatResponse>()
                            .unwrap();

                        // Extract the assistant's reply from the first choice
                        let openai_response = response
                            .choices
                            .into_iter()
                            .next()
                            .map(|choice| choice.message.content)
                            .unwrap_or_default();

                        cache.set(request_str.clone(), openai_response.clone());
                        openai_response
                    }
                    Some(result) => result.to_string(),
                };

                println!("OpenAI Chat Response: {}", response);
                response
            }
        }
    }
    pub fn emb(&self, content: &str) -> Vec<f32> {
        match &self.model_type {
            ModelType::Ollama { emb, .. } => {
                let request = OllamaEmbRequest {
                    model: emb.to_string(),
                    prompt: content.to_string()
                };

                // println!("Request: {:?}", request);
                let client = Client::builder()
                    .timeout(Duration::from_secs(60 * 10))
                    .build()
                    .unwrap();
                let response = client
                    .post(OLLAMA_EMB)
                    .json(&request)
                    .send()
                    .unwrap();
                // println!("Response: {:?}", response.text());
                let emb_response = response.json::<OllamaEmbResponse>().unwrap();
                emb_response.embedding
            },
            ModelType::OpenAI { .. } => {
                todo!("OpenAI does not support embeddings")
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaOptions {
    num_predict: i32,
    stop: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    done_reason: String,
    context: Vec<i64>,
    total_duration: i64,
    load_duration: i64,
    prompt_eval_count: i32,
    prompt_eval_duration: i64,
    eval_count: i32,
    eval_duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaEmbRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaEmbResponse {
    embedding: Vec<f32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: i32,
    temperature: f32,
    stop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String, // e.g., "user", "assistant", "system"
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIChatResponse {
    id: String,
    object: String,
    created: i64,
    choices: Vec<OpenAIChatChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIChatChoice {
    index: i32,
    message: ChatMessage, // Changed to include the message object
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}
