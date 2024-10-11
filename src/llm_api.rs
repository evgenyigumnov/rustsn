use crate::cache::Cache;
use crate::llm_prompt::Prompt;
use crate::{OLLAMA_API, OLLAMA_EMB, VERBOSE};
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
    Ollama { model: String, emb: String },
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
                if *VERBOSE.lock().unwrap() {
                    println!("Request: {}", request.prompt);
                }

                let response_opt = cache.get(&request_str);
                let response = match response_opt {
                    None => {
                        let client = Client::builder()
                            .timeout(Duration::from_secs(60 * 10))
                            .build()
                            .unwrap();
                        println!("Request to LLM in progress");

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
                    Some(result) => {
                        println!("Request already cached");
                        result.to_string()
                    }
                };

                if *VERBOSE.lock().unwrap() {
                    println!("Response: {}", response);
                }
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
                println!("Request to LLM in progress");
                if *VERBOSE.lock().unwrap() {
                    println!("Request: {}", user_prompt);
                }

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
                    Some(result) => {
                        println!("Request already cached");
                        result.to_string()
                    }
                };

                if *VERBOSE.lock().unwrap() {
                    println!("OpenAI Chat Response: {}", response);
                }
                response
            }
        }
    }
    pub fn emb(&self, content: &str, cache: &mut Cache) -> Vec<f32> {
        match &self.model_type {
            ModelType::Ollama { emb, .. } => {
                let request = OllamaEmbRequest {
                    model: emb.to_string(),
                    prompt: content.to_string(),
                };

                let request_str = serde_json::to_string(&request).unwrap();
                let response_opt = cache.get(&request_str);
                let response = match response_opt {
                    None => {
                        let client = Client::builder()
                            .timeout(Duration::from_secs(60 * 10))
                            .build()
                            .unwrap();
                        let response = client
                            .post(OLLAMA_EMB)
                            .json(&request)
                            .send()
                            .unwrap()
                            .json::<OllamaEmbResponse>()
                            .unwrap();
                        cache.set(
                            request_str.clone(),
                            serde_json::to_string(&response.embedding).unwrap(),
                        );
                        response.embedding
                    }
                    Some(result) => serde_json::from_str(&result).unwrap(),
                };
                response
            }
            ModelType::OpenAI { api_key } => {
                let request = OpenAIEmbRequest {
                    model: "text-embedding-ada-002".to_string(),
                    input: content.to_string(),
                };

                let request_str = serde_json::to_string(&request).unwrap();

                let response_opt = cache.get(&request_str);

                let response = match response_opt {
                    None => {
                        let client = Client::builder()
                            .timeout(Duration::from_secs(60 * 5))
                            .build()
                            .unwrap();

                        println!("Request to OpenAI Embeddings API in progress");

                        let api_response = match client
                            .post("https://api.openai.com/v1/embeddings")
                            .bearer_auth(api_key)
                            .json(&request)
                            .send()
                        {
                            Ok(resp) => resp,
                            Err(e) => {
                                eprintln!("Network error: {}", e);
                                return vec![];
                            }
                        };


                        let api_response = match api_response.json::<OpenAIEmbResponse>() {
                            Ok(json) => json,
                            Err(e) => {
                                eprintln!("Failed to parse JSON response: {}", e);
                                return vec![];
                            }
                        };

                        cache.set(
                            request_str.clone(),
                            serde_json::to_string(&api_response).unwrap(),
                        );
                        api_response.data[0].embedding.clone()
                    }
                    Some(result) => {
                        println!("Request already cached");
                        serde_json::from_str(&result).unwrap()
                    }
                };

                if *VERBOSE.lock().unwrap() {
                    println!("OpenAI Embedding Response: {:?}", response);
                }
                response.to_vec()
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
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIEmbRequest {
    model: String,
    input: String,
}


// {
//   "object": "list",
//   "data": [
//     {
//       "object": "embedding",
//       "index": 0,
//       "embedding": [
//         -0.010771754, 0.01454495, -0.0068650246, -0.008176398, -0.0108744735, 0.032322112, -0.020502627, -0.038156528, -0.024542892, -0.050482757, -0.00431418, -0.0102855535, -0.03295212, 0.018516736, 0.013709506, 0.019146742, 0.01706498, -0.02383071, 0.016722584, -0.018955002,
//         0.0041464064, -0.010737515, -0.016421277, 0.009943158, -0.0059063183, 0.008655752, 0.004225157, -0.009621306, 0.0076491097, 0.0073135626, 0.042292662, -0.019461745, 0.007991505, -0.019982187, -0.0034359363, -0.015599527, -0.000091376736, -0.008265422, 0.009922614, -0.020023275,
//         0.010559469, 0.021762643, -0.015476265, 0.0039409692, -0.027377924, 0.03402039, 0.027350532, -0.0353078, -0.029473383, 0.021091547, 0.019393267, 0.020406757, 0.010374576, -0.005930286, 0.007950418, -0.008847494, -0.01666778, 0.027104009, 0.014681908, -0.012559058, -0.0063000727,
//         0.009895223, -0.00925152, -0.007792916, 0.000288468, -0.016777368, -0.022543304, 0.017503245, -0.036786947, -0.014490167, 0.013969726, 0.013627331, 0.0015673143, -0.0120591605, 0.023036353, -0.0218996, -0.039170016, -0.021296985, -0.006262409, -0.023584185, 0.020530019,
//         0.0019704846, -0.0021707858, -0.0013892688, 0.00519756, 0.012723408, 0.026227476, 0.031719495, -0.024200497, 0.00013385514, 0.008457162, 0.0039478173, -0.0075463913, 0.017982598, 0.0078956345, -0.0061836583, 0.025556382, 0.013784833, -0.008313357, -0.018475648, 0.013442437,
//         0.01284667, -0.028459894, -0.007566935, -0.0023693752, 0.0070259506, -0.0353078, -0.004266245, 0.0073272586, 0.0058035995, -0.006553445, 0.012743951, 0.007909331, -0.024063539, 0.01657193, -0.041005254, 0.03941654, 0.0034359363, 0.0047216304, -0.003158596, -0.004389507,
//         0.026035735, 0.0076559577, -0.010217074, 0.024652459, -0.0069711674, -0.03150036, -0.014955824, 0.003896458, -0.012908301, 0.022598086, 0.034239527, 0.019311093, -0.023255486, -0.012737104, 0.03191124, -0.019201526, -0.004536737, -0.014462776, -0.028542068, -0.008374988,
//         0.043881375, -0.03160993, -0.004170374, 0.01049099, -0.00096127467, 0.015202349, 0.0037115645, 0.017571725, -0.021680467, 0.023912884, -0.01909196, 0.0052728867, 0.010675884, 0.025049636, 0.025611164, -0.019037176, 0.02510442, 0.013421894, -0.046209663, 0.02079024,
//         0.047990117, -0.02107785, 0.001370437, 0.00578648, 0.0147366915, -0.0014226523, -0.0055125635, -0.013613635, 0.0099705495, -0.030295132, 0.005687185, -0.032705594, 0.028103802, -0.008053136, 0.028158585, 0.0052112555, 0.00303191, -0.026761614, -0.00074856164, 0.008744774,
//         ...
//       ]
//     }
//   ],
//   "model": "text-embedding-ada-002",
//   "usage": {
//     "prompt_tokens": 6833,
//     "total_tokens": 6833
//   }
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIEmbResponse {
    data: Vec<OpenAIEmbData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIEmbData {
    embedding: Vec<f32>,
}

