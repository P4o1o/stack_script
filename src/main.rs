//
// main.rs - Entry point and REPL for sscript
//

use std::env;
// ...existing code...
use rustyline::error::ReadlineError;
use rustyline::{Editor, history::FileHistory};

use sscript::error::SscriptError;
use sscript::interpreter::{execute, load_file, ProgramState};
use sscript::stack_op::print_stack;

const VERSION: &str = "0.1.0";

fn print_usage() {
    println!(
        "\nUsage:\n\tsscript [-options] [File to load before the shell starts]\n\
        \targs are optionals:\n\n\
        Documentation available at https://p4o1o.github.io/stack_script/\n\n\
        options must be in this format: -v, -sv2m -sv, ... (the order doesn't matter)\n\
        options available:\n\
        \t-v\t\t print the last element of the stack after every input.\n\
        \t-v<size>\t print the last <size> elements of the stack after every input.\n\
        \t-h\t\t print this message.\n\
        \t-m\t\t load the math library before the shell starts\n\
        \t-p\t\t load the probability library before the shell starts\n\
        \t-s\t\t load the stack operations library before the shell starts\n\n"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut state = ProgramState::new();
    let mut print_size: usize = 0;
    let mut file_to_load: Option<String> = None;
    
    // Parse arguments
    if args.len() > 1 {
        let first_arg = &args[1];
        
        if first_arg.starts_with('-') {
            let mut chars = first_arg.chars().skip(1).peekable();
            
            while let Some(c) = chars.next() {
                match c {
                    'v' => {
                        // Check for numeric suffix
                        let mut num_str = String::new();
                        while let Some(&nc) = chars.peek() {
                            if nc.is_ascii_digit() {
                                num_str.push(nc);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        
                        if num_str.is_empty() {
                            print_size = 1;
                        } else {
                            print_size = num_str.parse().unwrap_or(1);
                        }
                    }
                    'h' => {
                        print_usage();
                        return;
                    }
                    'm' => {
                        if let Err(e) = load_file(&mut state, "math.sksp") {
                            eprintln!("Warning: Could not load math.sksp: {}", e);
                        }
                    }
                    's' => {
                        if let Err(e) = load_file(&mut state, "stackop.sksp") {
                            eprintln!("Warning: Could not load stackop.sksp: {}", e);
                        }
                    }
                    'p' => {
                        if let Err(e) = load_file(&mut state, "probability.sksp") {
                            eprintln!("Warning: Could not load probability.sksp: {}", e);
                        }
                    }
                    _ => {
                        print_usage();
                        std::process::exit(1);
                    }
                }
            }
            
            // Check for file argument after options
            if args.len() > 2 {
                file_to_load = Some(args[2].clone());
            }
        } else {
            // First argument is a file
            if args.len() > 2 {
                print_usage();
                std::process::exit(1);
            }
            file_to_load = Some(first_arg.clone());
        }
    }
    
    // Load initial file if specified
    if let Some(filename) = file_to_load {
        if let Err(e) = load_file(&mut state, &filename) {
            eprintln!("Error loading {}: {}", filename, e);
            std::process::exit(1);
        }
    }
    
    // Start REPL
    println!("STACK_SCRIPT v{}", VERSION);
    println!("-------------------------------------------");
    
    let mut rl = Editor::<(), FileHistory>::new().unwrap();
    let _ = rl.load_history(".sscript_history");
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.clone()).unwrap_or(false);
                match execute(&mut state, &line) {
                    Ok(()) => {}
                    Err(SscriptError::ProgramExit) => break,
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
                if print_size > 0 {
                    print_stack(&state.stack, print_size);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    let _ = rl.save_history(".sscript_history");
}
