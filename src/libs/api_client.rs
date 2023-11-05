use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

const HOST: &str = "https://api.openai.com/";
const VERSION: &str = "v1";

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChat {
    pub choices: Vec<ResponseChatChoice>,
    id: String,
    usage: ResponseUsage,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseUsage {
    prompt_tokens: u16,
    completion_tokens: u16,
    total_tokens: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChatChoice {
    pub message: ResponseChatMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChatMessage {
    role: String,
    pub content: String,
}

// ref: https://help.openai.com/en/articles/6654000-best-practices-for-prompt-engineering-with-openai-api

pub struct Agent {
    pub token: String,
    pub model: String,
    pub max_tokens: Option<u16>,
    pub client: Client,
    pub messages: Messages,
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}
pub struct Messages {
    conversation: Vec<Message>,
}
impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for msg in &self.conversation {
            write!(f, "{}: {}", msg.role, msg.content.replace('\n', ""))?;
        }
        writeln!(f)
    }
}

impl Agent {
    pub fn new(token: String, model: String) -> Agent {
        Agent {
            token,
            model,
            max_tokens: Some(1000),
            client: reqwest::Client::new(),
            messages: Messages {
                conversation: vec![],
            },
        }
    }

    pub fn set_system(&mut self, prompt: &str) {
        let message = Message {
            role: "system".to_string(),
            content: prompt.to_string(),
        };
        self.messages.conversation.push(message);
    }

    pub async fn chat(&mut self, message: &str) -> Result<String, reqwest::Error> {
        let role = "user".to_string();
        let msg = Message {
            role: role.clone(),
            content: message.to_string(),
        };
        self.messages.conversation.push(msg);

        let agent_content = self.send_request(&role, &self.messages).await.unwrap();
        let agent_msg = Message {
            role: "assistant".to_string(),
            content: agent_content.clone(),
        };
        self.messages.conversation.push(agent_msg);
        Ok(agent_content.clone())
    }

    async fn send_request(&self, role: &String, msgs: &Messages) -> Result<String, reqwest::Error> {
        let post_body = json!({
            "model": self.model,
            "messages": msgs.conversation,
            "max_tokens": self.max_tokens,
            "user": role
        });

        let response = self
            .client
            .post(format!(
                "{}{}{}",
                self::HOST,
                self::VERSION,
                "/chat/completions"
            ))
            .bearer_auth(self.token.clone())
            .json(&post_body)
            .send()
            .await
            .expect("Request error");
        let agent_response: ResponseChat = response.json().await.expect("JSON error");
        let agent_content = agent_response.choices[0].message.content.clone();
        Ok(agent_content)
    }
}
