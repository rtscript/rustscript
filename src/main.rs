use std::env::args;
use std::fs::File;
use std::io::{self, stdout, BufRead, Write};

mod error;
use error::*;

mod ast;
use ast::*;

mod parser;
use parser::*;

mod scanner;
use scanner::*;

mod object;
mod token;
mod token_type;
mod environment;
mod utility;
mod typechecker;
mod types;




pub fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: rustscript [script]");
            std::process::exit(64);
        }
    }
}


fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(buf).is_err() {
        // Ignore: error was already reported
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line);
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}

fn write_to_file(code: String) -> std::io::Result<()> {
    let mut file = File::create("target/index.js")?;
    file.write_all(code.as_bytes())?;
    Ok(())
}

fn run(source: String) -> Result<(), Problem> {

    //Tokenisation
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // for token in tokens {
    //     println!("{:?}", token);
    // }

    //Parse the Ast
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut code: String =  "".to_string();
    let mut tabs:usize = 0;
    for ast_tokens in ast {
        // println!("{:?}", &ast_tokens);
        match ast_tokens.ast_type {
            AstType::Fn => code += "function ",
            AstType::Identifier => code += ast_tokens.ast_lexeme(),
            AstType::LeftParen => code += "(",
            AstType::RightParen => code += ")",
            AstType::Comma => code += ", ",
            AstType::LeftBrace => {
                code += " {\n";
                tabs = tabs + 1;
                code += "\t".repeat(tabs).as_str();
            },
            AstType::Print => code += "console.log",
            AstType::String => {
                code += ast_tokens.ast_lexeme();
            },
            AstType::SemiColon => {
                code +=  ";\n";
                code += "\t".repeat(tabs).as_str();
            }
            AstType::RightBrace => {
                tabs = tabs - 1;
                code += "\n";
                code += "\t".repeat(tabs).as_str();
                code += "}\n";
            }
            AstType::Let => code += "let ",
            AstType::Number => code += ast_tokens.ast_lexeme(),
            AstType::Assign => code += " = ",
            AstType::Main => code += ")()", 
            _ => println!("Undefined"),
        }
    }

    write_to_file(code);

    Ok(())
}
