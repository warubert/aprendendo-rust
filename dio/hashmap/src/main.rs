use std::collections::HashMap;

#[derive(Debug)]
struct Funcionario {
    id: u32,
    nome: String,
    salario: f64,
}

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    match map.get("b") {
        Some(value) => println!("Valor encontrado: {}", value),
        None => println!("Valor não encontrado"),
    }

    match map.get("z") {
        Some(value) => println!("Valor encontrado: {}", value),
        None => println!("Valor não encontrado"),
    }

    let map2: HashMap<&str, Funcionario> = HashMap::from([
        ("Func1", Funcionario {
            id: 1,
            nome: String::from("João"),
            salario: 3000.0,
        }),
        ("Func2", Funcionario {
            id: 2,
            nome: String::from("Maria"),
            salario: 3500.0,
        }),
    ]);

    println!("{:#?}", map2);
}
