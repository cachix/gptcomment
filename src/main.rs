use clap::{Parser};
use std::path::PathBuf;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use std::env;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let client = Client::new(env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));

    for file in cli.files {
        if let Some(extension) = file.extension().and_then(|e| e.to_str()) {
            let lang = match extension {
                "js" => tree_sitter_javascript::language(),
                "sql" => tree_sitter_sql::language(),
                "rs" => tree_sitter_rust::language(),
                "py" => tree_sitter_python::language(),
                _ => {
                    println!("Unsupported file type {}, please contribute it to https://github.com/cachix/gptcomment", file.display());
                    continue
                }
            };
            parse_file(&file, lang, &client);
        } else {
            println!("File without extension: {}", file.display());
        }
    }
}

fn parse_file(file_path: &PathBuf, language: tree_sitter::Language, client: &Client) {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).expect("Error setting language");

    let code = std::fs::read(file_path).expect("Error reading file");

    let tree = parser.parse(&code, None).expect("Error parsing file");
    let root_node = tree.root_node();

    process_node(root_node, &code, client, file_path);
}


fn process_node(node: tree_sitter::Node, code: &Vec<u8>, client: &Client, file_path: &PathBuf) {
    if node.kind().contains("comment") {
        let content = node.utf8_text(code).unwrap();
        
        if content.contains("GPTComment:") {
            let instruction = content.split("GPTComment:").collect::<Vec<&str>>()[1];
            println!("Processing '{}' in {}", instruction, file_path.display());
            let msg = format!("If there are changes needed, respond only with the code snippet, no other text. If there are no changes needed, respond with OK.\n\n Apply {} to this file: {}", instruction, code.iter().map(|&c| c as char).collect::<String>());
            let req = ChatCompletionRequest::new(
                GPT4.to_string(),
                vec![chat_completion::ChatCompletionMessage {
                    role: chat_completion::MessageRole::user,
                    content: chat_completion::Content::Text(msg),
                    name: None,
                }],
            );
            let result = client.chat_completion(req).unwrap();
            let snippet = result.choices[0].message.content.as_ref().unwrap();
            if snippet != "OK" {
                println!("Updating file {}", file_path.display());
                std::fs::write(file_path, snippet).expect("Error writing file");
            } 
        }
    }  
    for child in node.children(&mut node.walk()) {
        process_node(child, code, client, file_path);
    }
}
