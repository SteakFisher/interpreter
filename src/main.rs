mod ast_printer;
mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod token_type;
mod stmt;
mod util;

use crate::ast_printer::AstPrinter;
use crate::expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::interpreter::Interpreter;
use crate::token::Token;
use crate::token_type::LiteralValue;
use parser::Parser;
use scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};
use crate::util::Utils;

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
            } else {
                println!("EOF  null");
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                scanner.scan_tokens();

                if scanner.has_error() {
                    std::process::exit(65);
                }

                let mut expr = Parser::new(scanner.get_tokens());
                let ast_printer = AstPrinter {};
                let expression = expr.expression().unwrap_or_else(|err| {
                    eprintln!("Runtime error: {}", err);
                    std::process::exit(65);
                });

                println!("{}", ast_printer.print(expression));
            } else {
                println!("EOF  ");
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                scanner.scan_tokens();

                if scanner.has_error() {
                    std::process::exit(65);
                }

                let mut expr = Parser::new(scanner.get_tokens());

                let expression = expr.expression().unwrap_or_else(|err| {
                    eprintln!("Runtime error: {}", err);
                    std::process::exit(65);
                });;

                let interpreter = Interpreter::new();
                let literal_value = Utils::print_literal(&interpreter
                    .interpret_expression(&Box::from(expression))
                    .unwrap_or_else(|err| {
                        eprintln!("Runtime error: {}", err);
                        std::process::exit(70);
                    }));

                println!("{}", literal_value);
            }
        }
        "run" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                scanner.scan_tokens();

                if scanner.has_error() {
                    std::process::exit(65);
                }

                let mut expr = Parser::new(scanner.get_tokens());

                let expression = expr.parse().unwrap_or_else(|err| {
                    eprintln!("Runtime error: {}", err);
                    std::process::exit(65);
                });

                let interpreter = Interpreter::new();
                interpreter
                    .interpret(&Box::from(expression))
                    .unwrap_or_else(|err| {
                        eprintln!("Runtime error: {}", err);
                        std::process::exit(70);
                    })
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
