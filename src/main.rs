use std::env;

mod ast;
mod parser;
mod lexer;

fn get_program_contents() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <PROGRAM> <input.c>");
        std::process::exit(1);
    }
    std::fs::read_to_string(&args[1]).unwrap()
}

fn main() {
    let input = get_program_contents();
    let tokens = lexer::tokenize(&input);

    match parser::parse_program(tokens) {
        Err(parse_error) => {
            println!("Parse error: {:?}", parse_error);
        },
        Ok(program) => {
            println!("Parsing OK. Parsed {} declarations.", program.len());
            println!("Result = {:?}", program);
        }
    }
}
