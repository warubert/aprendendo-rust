use std::env;
use dotenv::dotenv;

fn main() {
    let key = "AAA";
    std::env::set_var(key, "123");

    // remove
    env::remove_var(key);

    match env::var(key) {
        Ok(val) => println!("{}: {}", key, val),
        Err(e) => println!("Erro {}: {}", key, e),
    }

    // exemplo 2 - lendo do cli
    // CLI_ARG=TEST cargo run -q
    let cli_arg = env::var("CLI_ARG");

    match cli_arg {
        Ok(val) => println!("CLI_ARG: {}", val),
        Err(e) => println!("Erro CLI_ARG: {}", e),
    }

    //exemplo 3 - lendo do .env
    dotenv().ok();

    let api_key = env::var("API_KEY");

    match api_key {
        Ok(val) => println!("API_KEY: {}", val),
        Err(e) => println!("Erro API_KEY: {}", e),
    }
}
