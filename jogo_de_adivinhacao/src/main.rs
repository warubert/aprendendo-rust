use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe um número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    // println!("O número secreto é: {numero_secreto}");

    loop {
        println!("Digite um númreo entre 1 e 100");
    
        let mut numero = String::new();
    
        io::stdin()
            .read_line(&mut numero)
            .expect("Erro ao ler a linha");
    
        let numero: u32 = match numero.trim().parse() {
            Ok(num) => num,
            Err (_) => {
                println!("Por favor, digite um número válido.");
                continue;
            }
        };    
        println!("Você digitou: {numero}");
    
        match numero.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
