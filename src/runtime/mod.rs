use std::io::{self, BufRead, Write};
use std::fs;
use std::process;
use crate::lexer;
pub mod errors;

fn retrieve_file_path(args: &[String]) -> Result<String, &'static str> {
    if args.len() > 2 {
        Err("Too many arguments.")
    } else if args.len() == 2 {
        Ok(args[1].clone())
    } else {
        Ok("".to_string())
    }
}

fn run_lovelace_from_file(file_path: String) {
    let source = match fs::read_to_string(file_path) {
        Ok(source_string) => source_string,
        Err(err) => {
            println!("Problem retrieving file: {err}");
            process::exit(66); //EX_NOINPUT as in file doesnt exo=ist or not readable
        },
    };

    run(source).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(65); //EX_DATAERR input data in the file is incorrect
    });

}

fn run_lovelace_repl() {
    println!("\nWelcome to the lovelace interactive environment,\nWrite some lovelace code and see what happens!\n");
    let quit_condition = "quit".to_string();
    let mut source;
    loop {
        print!("lovelace > ");
        std::io::stdout().flush().unwrap();
        source = io::stdin().lock().lines().next().unwrap().unwrap();
        if source == quit_condition {
            break
        } else {
            run(source)
            .unwrap_or_else(|err| {
                println!("{err}");
            });
        }
    }
}

fn run(source: String) -> Result<(), String> {    
    let mut lexer = lexer::Lexer::new(source);
    let tokens = match lexer.scan_tokens() {
        Err(err) => return Err(err),
        Ok(vector_of_tokens) => vector_of_tokens,
    };

    for token in tokens.iter() {
        print!("{}\n", token);
    }
    Ok(())
}

pub fn start_lovelace(args: &[String]) {
    let file_path = retrieve_file_path(&args)
    .unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(64); //EX_USAGE the lovelace interpreter was used incorrectly
    });

    if file_path.is_empty() {
        run_lovelace_repl();
    } else {
        run_lovelace_from_file(file_path);
    }
}

