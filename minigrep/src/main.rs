use std::env;
use std::fs;
use std::process;
use std::error::Error;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema: {}", err);
        process::exit(1)
    });

    println!("Procurando por: {}", config.query);
    println!("No arquivo: {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Erro: {}", e);
        process::exit(1);
    }
}