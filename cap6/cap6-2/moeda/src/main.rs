#[derive(Debug)]
enum Estado {
    Alabama,
    Alaska,
    // ... etc
}

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter(Estado),
}
fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter(estado) => {
            println!("Quarter do estado {:?}!", estado);
            25
        },
    }
}
fn main() {
    let moeda = Moeda::Quarter(Estado::Alabama);
    println!("Valor em cents: {}", valor_em_cents(moeda));
}
