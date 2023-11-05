use std::env;

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub model: String,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        let token = env::var("OPENAI_API_KEY")
            .map_err(|_| "The OPENAI_API_KEY environment variable is not set.".to_string())?;

        let model =
            env::var("OPENAI_DEFAULT_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

        Ok(Self { token, model })
    }
}
