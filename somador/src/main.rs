use std::io;

fn main() {
    println!("{:-^40}", "Calculadora");
    let mut s = String::new();
    let banner = 
        "Digite uma sequencia de numeros \n\
        separados por virgula \n\
        exemplo: 1,2,3,4,45";

    println!("{banner}");

    io::stdin()
        .read_line(&mut s)
        .expect("Erro ao ler a entrada");

    let nums: Vec<i32> = s
        .split(',')
        .map(|x| x.trim().parse().expect("Erro ao converter para inteiro"))
        .collect();

    let resultado: i32 = nums.iter().sum();

    println!("Você digitou: {:?}", nums);
    println!("Resultado: {}", resultado);

    println!("{:-^40}", "Fim da Calculadora");
}