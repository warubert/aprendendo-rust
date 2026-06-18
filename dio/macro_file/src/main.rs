use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

macro_rules! cria_struct {
    ($nome:ident { $($campo:ident: $tipo:ty),* $(,)?}) => {
        #[derive(Debug, Serialize, Deserialize)]
        struct $nome {
            $($campo: $tipo),*
        }
    };
}

cria_struct! {
    Cliente {
        id: u32,
        nome: String,
        cpf: String,
    }
}

fn main() {
    let data = fs::read_to_string("clientes.json").expect("Unable to read file");

    let clientes: Vec<Cliente> = serde_json::from_str(&data).expect("JSON was not well-formatted");

    for cliente in clientes {
        println!("{:?}", cliente);
    }
}
