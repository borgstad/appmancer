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
    Sh {
        // Add specific arguments for the `sh` subcommand if needed
        text: String,
    },
}

// const TERMINAL: &str = include_str!("../resources/terminal-corrector.txt");
const TERMINAL: &str = include_str!("../resources/terminal-helper.txt");
const DEVELOPER: &str = include_str!("../resources/developer.txt");

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
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
