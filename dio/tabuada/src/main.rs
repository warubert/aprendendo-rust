use std::io;

fn main() {
    loop{
        println!("Digite o valor da tabuada ou X para sair:");
        let mut tabuada: String = String::new();
    
        io::stdin()
            .read_line(&mut tabuada)
            .expect("Falha ao ler a entrada");

        match tabuada.trim() {
            "X" | "x" => {
                println!("Saindo...");
                break;
            },
            _ => (),
        }
    
        let tabuada: u32 = tabuada.trim().parse().expect("Por favor, digite um número válido");
    
        for i in 1..=10 {
            println!("{} x {} = {}", tabuada, i, tabuada * i);
        }
    }
}
