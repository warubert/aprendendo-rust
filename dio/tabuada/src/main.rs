use std::io;

fn main() {
    loop{
        println!(r#"
            Digite uma opção
            1 - Soma
            2 - Subtracao
            3 - Tabuada
            0 - Sair
        "#);

        let mut opcao: String = String::new();
    
        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler a entrada");

        let opcao: u8 = opcao.trim().parse().expect("Por favor, digite um número válido");

        match opcao {
            1 => println!("Soma: {}", soma()),
            2 => println!("Subtração: {}", subtracao()),
            3 => tabuada(),
            0 => {
                println!("Saindo...");
                break;
            },
            _ => println!("Opção inválida"),
        }
    }
}

fn soma() -> i32 {
    println!("Digite o primeiro número:");
    let mut num1: String = String::new();
    
    io::stdin()
        .read_line(&mut num1)
        .expect("Falha ao ler a entrada");

    let num1: i32 = num1.trim().parse().expect("Por favor, digite um número válido");

    println!("Digite o segundo número:");
    let mut num2: String = String::new();
    
    io::stdin()
        .read_line(&mut num2)
        .expect("Falha ao ler a entrada");

    let num2: i32 = num2.trim().parse().expect("Por favor, digite um número válido");

    num1 + num2
}

fn subtracao() -> i32 {
    println!("Digite o primeiro número:");
    let mut num1: String = String::new();
    
    io::stdin()
        .read_line(&mut num1)
        .expect("Falha ao ler a entrada");

    let num1: i32 = num1.trim().parse().expect("Por favor, digite um número válido");

    println!("Digite o segundo número:");
    let mut num2: String = String::new();
    
    io::stdin()
        .read_line(&mut num2)
        .expect("Falha ao ler a entrada");

    let num2: i32 = num2.trim().parse().expect("Por favor, digite um número válido");

    num1 - num2
}

fn tabuada() {
    println!("Digite o valor da tabuada ou X para sair:");
    let mut tabuada: String = String::new();
    
    io::stdin()
        .read_line(&mut tabuada)
        .expect("Falha ao ler a entrada");

    match tabuada.trim() {
        "X" | "x" => {
            println!("Saindo...");
            return;
        },
        _ => (),
    }
    
    let tabuada: u32 = tabuada.trim().parse().expect("Por favor, digite um número válido");
    
    for i in 1..=10 {
        println!("{} x {} = {}", tabuada, i, tabuada * i);
    }
}