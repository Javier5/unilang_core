use std::env;
use std::fs;
use std::process;
use std::time::Instant;

mod lexer;
mod parser;
mod ast;

use lexer::Lexer;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: unilang_core <archivo_fuente>");
        process::exit(1);
    }

    let file_path = &args[1];

    let source_code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error al leer el archivo '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    println!("⚡ [unilang_core] Archivo cargado: {} bytes", source_code.len());

    let mut lexer = Lexer::new(&source_code);
    loop {
        let token = lexer.next_token();
        println!("Token encontrado: {:?}", token);
        if token == lexer::Token::EOF {
            break;
        }
    }

    println!("⚡ [unilang_core] Tiempo total de ejecución: {:?}", start_time.elapsed());
}
