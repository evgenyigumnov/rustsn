use crate::cache::Cache;
use crate::llm_prompt::Prompt;
use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_template,
    llm::openai::{OpenAI, OpenAIConfig, OpenAIModel},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};
use tokio::runtime::Runtime;

pub struct LLMApi {
    model_type: ModelType,
}

#[derive(Debug, PartialEq)]
pub enum ModelType {
    Ollama,
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
            ModelType::Ollama => {
                // Default implementation for Ollama integration
                "Ollama integration is not implemented yet.".to_string()
            }
            ModelType::OpenAI { api_key } => {
                let rt = Runtime::new().unwrap();
                let user_prompt = prompt.create(prompt_template, params);

                let open_ai = OpenAI::default()
                    .with_model(OpenAIModel::Gpt4oMini.to_string())
                    .with_config(OpenAIConfig::default().with_api_key(api_key));

                let prompt_template = message_formatter![
                    fmt_message!(Message::new_system_message(
                        "You are a helpful assistant."
                    )),
                    fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                        "{}", "input"
                    )))
                ];

                let chain = LLMChainBuilder::new()
                    .prompt(prompt_template)
                    .llm(open_ai)
                    .build()
                    .unwrap();

                let result = rt.block_on(async {
                    chain
                        .invoke(prompt_args! {
                            "input" => user_prompt,
                        })
                        .await
                });

                match result {
                    Ok(response) => response.to_string(),
                    Err(e) => panic!("Error invoking LLMChain: {:?}", e),
                }
            }
        }
    }
}
