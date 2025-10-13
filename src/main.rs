mod ast;
mod evaluator;
mod lexer;
mod parser;

use parser::Parser;

fn main() {
    let input = "10 + 5 * 2"; // Try changing this! e.g., "(10 + 5) * 2"
    println!("Evaluating: {}", input);

    let mut parser = Parser::new(input);

    match parser.parse_expression() {
        Ok(ast) => {
            println!("AST: {:#?}", ast);
            match evaluator::eval(&ast) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => eprintln!("Runtime Error: {}", e),
            }
        }
        Err(e) => eprintln!("Parse Error: {}", e),
    }
}