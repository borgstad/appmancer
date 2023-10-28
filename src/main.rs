use clap::{command, Parser, Subcommand};
use serde_json::json;
use std::io;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
mod libs;
use colored::Colorize;
use include_dir::{include_dir, Dir};
use libs::api_client::{Agent, ResponseChat};
use rustyline::error::ReadlineError;
use rustyline::{history, DefaultEditor};
use std::io::prelude::*;
use std::process;

static SYSTEMS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");

#[tokio::main]
async fn main() {
    libs::logger::init();
    let user_conf = libs::config::load_config();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();

    let developer_files_in_project = SYSTEMS
        .get_file("developer.txt")
        .unwrap()
        .contents_utf8()
        .unwrap();
    let mut rl = DefaultEditor::with_config(
        rustyline::Config::builder()
            .auto_add_history(true)
            .completion_type(rustyline::CompletionType::List)
            .max_history_size(10000)
            .unwrap()
            .build(),
    )
    .unwrap();
    rl.load_history("history.txt").unwrap();

    let readline = rl
        .readline("Please enter the path to you project: ")
        .unwrap();
    let mut project_path = Path::new(&readline);
    let mut files_in_project: String = "".to_string();
    if project_path.exists() {
        project_path = Path::new(&readline);
        let f = list_files_with_exclusions(
            project_path,
            vec![".git", "target", ".gitignore", "Cargo.lock"],
        )
        .unwrap();
        if f.len() == 0 {
            let files_in_project = "The project currently have no files".to_string();
        } else {
            let files_in_project = "The project currently have the following files:\n".to_string();
            let files_in_project = files_in_project + &f.join("\n");
        }
    } else {
        println!("Project does not exist, exiting");
        process::exit(0);
    }

    let mut reg = handlebars::Handlebars::new();
    reg.register_template_string("system_developer", developer_files_in_project);
    let system_developer = reg
        .render("system_developer", &json!({"files": files_in_project}))
        .unwrap();

    let mut agent_developer = Agent::new(user_conf.token, user_conf.model, &system_developer);
    rl.save_history("history.txt").unwrap();

    loop {
        let readline = rl.readline("User: ").unwrap();
        let role = "user".to_string();

        let result = agent_developer
            .chat(&readline, &role)
            .await
            .expect("Request error");
        println!("{}: {}", "Agent: ".green(), result.bold());
        rl.save_history("history.txt").unwrap();
    }
}

fn is_excluded(entry: &DirEntry, exclude_paths: &[&str]) -> bool {
    exclude_paths
        .iter()
        .any(|&exclude_path| entry.path().to_string_lossy().contains(exclude_path))
}

fn list_files_with_exclusions<P: AsRef<Path>>(
    path: P,
    exclude_paths: Vec<&str>,
) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok()) // Ensure we're using walkdir's Result type here
        .filter(|e| e.file_type().is_file() && !is_excluded(e, &exclude_paths))
    {
        files.push(
            entry
                .path()
                .to_str()
                .ok_or(io::Error::new(
                    io::ErrorKind::Other,
                    "Path conversion error",
                ))?
                .to_string(),
        );
    }

    Ok(files)
}
