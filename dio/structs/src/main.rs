
#[derive(Debug)]
struct Funcionario {
    id: u32,
    nome: String,
    salario: f64,
}

impl Funcionario {
    fn new(id: u32, nome: String, salario: f64) -> Self {
        Funcionario { id, nome, salario }
    }

    fn oi(&self) {
        println!("ID: {}", self.id);
        println!("Nome: {}", self.nome);
        println!("Salário: {}", self.salario);
    }
}

#[derive(Debug)]
struct FuncionarioNull {
    id: Option<u32>, //ou Option<u32,
    nome: Option<String>,
    salario: Option<f64>,
}

fn main() {
    println!("Hello, world!");

    let funcionario: Funcionario = Funcionario {
        id: 1,
        nome: String::from("João"),
        salario: 3000.0,
    };

    println!("ID: {}", funcionario.id);
    println!("Nome: {}", funcionario.nome);
    println!("Salário: {}", funcionario.salario);

    println!("{}", "-".repeat(20));

    let func: (u32, String, f64) = (2, String::from("Maria"), 3500.0);

    println!("ID: {}", func.0);
    println!("Nome: {}", func.1);
    println!("Salário: {}", func.2);

    let llista_funcionarios: Vec<Funcionario> = vec![
        Funcionario {
            id: 1,
            nome: String::from("João"),
            salario: 3000.0,
        },
        Funcionario {
            id: 2,
            nome: String::from("Maria"),
            salario: 3500.0,
        },
    ];

    println!("{}", "-".repeat(20));

    for funcionario in llista_funcionarios {
        println!("ID: {}", funcionario.id);
        println!("Nome: {}", funcionario.nome);
        println!("Salário: {}", funcionario.salario);
    }

    println!("{}", "-".repeat(20));

    let mut funcionario_null: FuncionarioNull = builder_funcionario_null();
    funcionario_null.id = Some(3);
    // funcionario_null.nome = Some(String::from("Carlos"));
    funcionario_null.salario = Some(4000.0);

    println!("ID: {}", funcionario_null.id.unwrap_or(0));
    println!("Nome: {}", funcionario_null.nome.unwrap_or(String::from("Desconhecido")));
    println!("Salário: {}", funcionario_null.salario.unwrap_or(0.0));

    println!("{}", "-".repeat(20));

    println!("{:?}", funcionario);
    println!("{:#?}", funcionario);

    println!("{}", "-".repeat(20));
    funcionario.oi();
}

fn builder_funcionario_null() -> FuncionarioNull {
    FuncionarioNull { 
        id: None, 
        nome: None, 
        salario: None
    }
}