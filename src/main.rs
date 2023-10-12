use std::process;
use std::env;
use pagal::Interpreter;

fn main() {
    // get command line arguments
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }
    
    let interpreter = Interpreter::new();
    if let Err(e) = interpreter.run(arguments) {
        eprintln!("Error occured: {}", e);
    }
}
