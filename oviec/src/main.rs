use oviec::{Compiler, OvieResult};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file.ov>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    match run_file(filename) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

fn run_file(filename: &str) -> OvieResult<()> {
    let source = fs::read_to_string(filename)
        .map_err(|e| oviec::OvieError::io_error(format!("Could not read file '{}': {}", filename, e)))?;
    
    let mut compiler = Compiler::new();
    compiler.debug = true; // Enable debug output
    compiler.compile_and_run(&source)?;
    
    Ok(())
}
