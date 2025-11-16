mod ast;
mod evaluator;
mod lexer;
mod parser;

use parser::Parser;
use evaluator::Environment;

fn main() {
    // Example 1: Simple variable assignment and usage
    let input1 = "x = 10";
    println!("=== Example 1 ===");
    println!("Evaluating: {}", input1);
    evaluate(input1);

    // Example 2: Assignment with expression on right side
    let input2 = "y = 5 + 3";
    println!("\n=== Example 2 ===");
    println!("Evaluating: {}", input2);
    evaluate(input2);

    // Example 3: Using variables in expressions
    let input3 = "x = 10; y = x + 5; z = x * y";
    println!("\n=== Example 3 ===");
    println!("Evaluating: {}", input3);
    evaluate_multiple(input3);

    // Example 4: Chained assignments
    let input4 = "a = b = c = 5";
    println!("\n=== Example 4 ===");
    println!("Evaluating: {}", input4);
    evaluate(input4);
}

fn evaluate(input: &str) {
    let mut parser = Parser::new(input);
    let mut env = Environment::new();

    match parser.parse_expression() {
        Ok(ast) => {
            println!("AST: {:#?}", ast);
            match evaluator::eval(&ast, &mut env) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => eprintln!("Runtime Error: {}", e),
            }
        }
        Err(e) => eprintln!("Parse Error: {}", e),
    }
}

fn evaluate_multiple(input: &str) {
    // Split by semicolon to handle multiple statements
    let statements: Vec<&str> = input.split(';').map(|s| s.trim()).collect();
    let mut env = Environment::new();

    for statement in statements {
        if statement.is_empty() {
            continue;
        }

        let mut parser = Parser::new(statement);
        match parser.parse_expression() {
            Ok(ast) => {
                println!("\nStatement: {}", statement);
                println!("AST: {:#?}", ast);
                match evaluator::eval(&ast, &mut env) {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => eprintln!("Runtime Error: {}", e),
                }
            }
            Err(e) => eprintln!("Parse Error: {}", e),
        }
    }
}