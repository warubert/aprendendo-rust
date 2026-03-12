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

fn main() {
    let mut contagem = 0;
    let moeda = Moeda::Penny;
    if let Moeda::Quarter(estado) = moeda {
        println!("Quarter do estado {:?}!", estado);
    } else {
        contagem += 1;
    }

    println!("Contagem: {}", contagem);
}
