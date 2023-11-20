# Appmancer

![](appmancer-sh.gif)

Appmancer is a command-line interface (CLI) tool designed to enhance the developer's workflow by providing code refactoring suggestions and generating bash commands. It integrates with an AI backend, harnessing the power of machine learning models to analyze and process code efficiently.
## Features
* Bash Command Synthesis: Input a description of the task you want to perform in bash, and Appmancer will generate the corresponding command.
* Git commit message: Create a semantic commit git message based on diff (`--staged` flag supported), and last 10 git commit messages.

## Installation
Ensure you have Rust and Cargo installed on your machine to get started with Appmancer.
Follow these steps to install Appmancer:
```
git clone https://github.com/borgstad/appmancer.git
cd appmancer
cargo build --release
```
The built executable will be located at target/release/.

## Usage
Execute Appmancer using the following commands:
```
./appmancer sh "describe your bash task"

./appmancer git

# Or if files have been staged via `git add`:
./appmancer git --staged
```

Configuration

Appmancer requires the following environment variables:

* OPENAI_API_KEY: Your personal OpenAI API key.
* OPENAI_DEFAULT_MODEL: The preferred OpenAI model (defaults to "gpt-3.5-turbo" if not specified).

You can set these directly in your environment or through a .env file in the project's root directory.
