fn main() {

    let nome: &str = "Will";

    let ano_nascimento: u16 = 1989;
    let mes_nascimento: u16 = 6;
    let dia_nascimento: u16 = 22;
    let ano_atual: u16 = 2026;
    let mes_atual: u16 = 6;
    let dia_atual: u16 = 3;

    let mut idade: u16 = ano_atual - ano_nascimento;
    if mes_atual < mes_nascimento {
        idade -= 1;
    } else if dia_atual < dia_nascimento {
        idade -= 1;
    }

    println!("Ola, {}! Voce tem {} anos.", nome, idade);
    println!("Hello, world!");
}
