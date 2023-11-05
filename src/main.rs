use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command};
use tokio::main;

mod libs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate code as diffs
    Dev {
        /// Where is the file
        file_path: PathBuf,
        // What should be done on the file
        // text: String,
    },
    /// Generate bash code
    Sh {
        /// What code should be generated
        text: String,
    },
}

const TERMINAL: &str = include_str!("../resources/terminal-helper.txt");
const DEVELOPER: &str = include_str!("../resources/developer.txt");

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dev { file_path } => {
            // Handle the 'dev' subcommand, working with the provided file path
            develop(file_path).await;
            // Here you would call a function that handles the 'dev' command logic
        }
        Commands::Sh { text } => {
            terminal_corrector(&text).await;
        }
    }
}

async fn terminal_corrector(text: &str) {
    let config = libs::config::load_config(); // Assuming this does not fail.
    let mut agent = libs::api_client::Agent::new(config.token, config.model, TERMINAL);

    // The following assumes agent.chat returns a Result type.
    match agent.chat(text).await {
        Ok(response) => {
            let bash_command = response.replace("```", "").trim().to_string();
            println!("{}", bash_command);
            print!("Would you like to execute this command? (y/n): ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            if ["y", "yes"].contains(&user_input.trim().to_lowercase().as_str()) {
                match Command::new("sh").arg("-c").arg(&bash_command).status() {
                    Ok(_) => {}
                    Err(e) => eprintln!("Failed to execute command: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

async fn develop(path: &PathBuf) {
    let config = libs::config::load_config(); // Assuming this does not fail.
    let mut agent = libs::api_client::Agent::new(config.token, config.model, DEVELOPER);

    let code = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    let changes = format!(
        "IMPORTANT: make the diff changes to this file: {}",
        path.display()
    );
    // let chat_text = format!("{}\n{}\n```rust\n{}\n```", text, changes, code);
    let chat_text = code;
    // println!("{}", chat_text);

    // The following assumes agent.chat returns a Result type.
    match agent.chat(&chat_text).await {
        Ok(response) => {
            // let bash_command = response.replace("```", "").trim().to_string();
            println!("{}", response);
        }
        Err(e) => {
            println!("hello");
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
