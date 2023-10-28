use std::env;

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub model: String,
}

pub fn load_config() -> Config {
    let token_env = "OPENAI_API_KEY";

    let token = env::var(token_env)
        .unwrap_or_else(|_| panic!("The {:?} environment is not set.", token_env))
        .to_string();
    let model = env::var("OPENAI_DEFAULT_MODEL").unwrap_or("gpt-3.5-turbo".to_string());

    Config { token, model }
}
