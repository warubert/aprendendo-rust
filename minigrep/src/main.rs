use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema: {}", err);
        process::exit(1)
    });

    println!("Procurando por: {}", config.query);
    println!("No arquivo: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Algo deu errado ao ler o arquivo");

    println!("{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Faltam argumentos");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}