mod error;
mod expr;
mod scanner;
mod token;
mod token_type;
mod ast_printer;

use scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};
use crate::ast_printer::AstPrinter;
use crate::expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::token::Token;
use crate::token_type::LiteralValue;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                scanner.scan_tokens();

                print!("{}", scanner);

                if scanner.has_error() {
                    std::process::exit(65);
                }

                let expression: Expr = Expr::Binary(Binary {
                    left: Box::from(Expr::Unary(Unary {
                        operator: Token {
                            token_type: token_type::TokenType::Minus,
                            lexeme: "-".to_string(),
                            literal: None,
                            line: 1,
                        },
                        right: Box::from(Expr::Literal(Literal { value: LiteralValue::Number(123.00) })),
                    })),
                    operator: Token {
                        token_type: token_type::TokenType::Minus,
                        lexeme: "-".to_string(),
                        literal: None,
                        line: 1,
                    },
                    right: Box::from(Expr::Grouping(Grouping { expression: Box::from(Expr::Literal(Literal { value: LiteralValue::Number(45.67) })) })),
                });
                let ast_printer = AstPrinter {};

                println!("{}", ast_printer.print(expression));
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
