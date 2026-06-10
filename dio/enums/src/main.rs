enum Tipo{
    Funcionario,
    Gerente,
    Diretor,
}

struct Funcionario{
    id: u32,
    nome: String,
    salario: f64,
    tipo: Tipo,
}

fn main() {
    let funcionario: Funcionario = Funcionario {
        id: 1,
        nome: String::from("João"),
        salario: 3000.0,
        tipo: Tipo::Funcionario,
    };

    match funcionario.tipo {
        Tipo::Funcionario => println!("Funcionário"),
        Tipo::Gerente => println!("Gerente"),
        Tipo::Diretor => println!("Diretor"),
    }
}
