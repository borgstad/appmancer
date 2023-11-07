use clap::{Parser, Subcommand};
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command};
use tempfile::NamedTempFile;

mod libs;
use libs::api_client::Agent;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ideas for recfactoring code
    Refactor {
        /// File location
        file_path: PathBuf,
    },
    /// Generate bash code
    Sh {
        /// Description of bash code
        text: String,
    },
}

const TERMINAL: &str = include_str!("../resources/terminal-helper.txt");
const DEVELOPER: &str = include_str!("../resources/developer.txt");

#[tokio::main]
async fn main() {
    let config = match libs::config::Config::new() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let cli = Cli::parse();
    let mut agent = Agent::new(config.token, config.model);

    match &cli.command {
        Commands::Refactor { file_path } => {
            // Handle the 'dev' subcommand, working with the provided file path
            develop(&mut agent, file_path).await;
            // Here you would call a function that handles the 'dev' command logic
        }
        Commands::Sh { text } => {
            terminal_corrector(&mut agent, text, &config.editor).await;
        }
    }
}

async fn terminal_corrector(agent: &mut Agent, text: &str, editor: &str) {
    agent.set_system(TERMINAL);
    match agent.chat(text).await {
        Ok(response) => {
            let bash_command = response.replace("```", "").trim().to_string();
            println!("{}", bash_command);
            println!("Would you like to execute this command? [y/n/m]");

            enable_raw_mode().expect("Failed to enable raw mode");
            let mut command_executed = false;

            while !command_executed {
                if let Event::Key(key_event) = read().expect("Failed to read event") {
                    match key_event.code {
                        KeyCode::Char('y') => {
                            disable_raw_mode().expect("Failed to disable raw mode");
                            execute_command(&bash_command);
                            command_executed = true;
                        }
                        KeyCode::Char('n') => {
                            disable_raw_mode().expect("Failed to disable raw mode");
                            command_executed = true;
                        }
                        KeyCode::Char('m') => {
                            disable_raw_mode().expect("Failed to disable raw mode");
                            let mut file =
                                tempfile::NamedTempFile::new().expect("Failed to create temp file");
                            writeln!(file, "{}", bash_command)
                                .expect("Failed to write to temp file");

                            Command::new(editor)
                                .arg(file.path())
                                .status()
                                .expect("Failed to start editor");
                            let modified_command = std::fs::read_to_string(file.path())
                                .expect("Failed to read temp file");
                            println!("{}", modified_command);
                            execute_command(&modified_command);
                            command_executed = true;
                        }
                        _ => {}
                    }
                }
            }
            disable_raw_mode().expect("Failed to disable raw mode");
        }
        Err(e) => {
            disable_raw_mode().expect("Failed to disable raw mode");
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn execute_command(command: &str) {
    match Command::new("sh").arg("-c").arg(command).status() {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
}

async fn develop(agent: &mut Agent, path: &PathBuf) {
    agent.set_system(DEVELOPER);
    let code = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    let chat_text = code;
    match agent.chat(&chat_text).await {
        Ok(response) => {
            println!("{}", response);
        }
        Err(e) => {
            println!("hello");
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
