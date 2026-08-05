use std::io;

fn main() {
    println!("Bem vindo ao Uppercaser!");
    println!("Digite uma frase e pressione enter:");

    let mut frase = String::new();
    io::stdin().read_line(&mut frase)
        .expect("Falha ao ler a linha");

    let frase_maiuscula = frase.trim().to_uppercase();
    println!("Frase em maiúsculas: {}", frase_maiuscula);
    println!("Pressione enter para sair...");
    let mut sair = String::new();
    io::stdin().read_line(&mut sair).expect("Falha ao ler a linha");
}
