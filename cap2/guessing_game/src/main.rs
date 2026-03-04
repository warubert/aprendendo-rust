use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");

    let n_secreto = rand::thread_rng().gen_range(1..=100);

    // println!("O número secreto é: {}", n_secreto);

    loop {
        println!("Digite um número entre 1 e 100.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler linha");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número válido.");
                continue;
            }
        };
    
        println!("Voce digitou: {}", guess);
    
        match guess.cmp(&n_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
