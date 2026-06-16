use std::io;

fn main() {
    // Lê a linha de entrada do usuário
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    // Divide a entrada em partes e faz o parse dos valores
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("Entrada inválida. Certifique-se de digitar saldo, operação e valor.");
        return;
    }

    let saldo: u32 = parts[0].parse().expect("Saldo inválido");
    let operacao = parts[1];
    let valor: u32 = parts[2].parse().expect("Valor inválido");

    match operacao {
        "deposit" => {
            let novo_saldo = saldo + valor;
            println!("{}", novo_saldo);
        }
        "withdraw" => {
            if valor > saldo {
                println!("Insufficient funds");
            } else {
                let novo_saldo = saldo - valor;
                println!("{}", novo_saldo);
            }
        }
        _ => println!("Operação inválida"),
    }
}