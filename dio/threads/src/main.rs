use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Cliente {
    id: u32,
    nome: String,
}

struct Produto {
    id: u32,
    nome: String,
}

struct Pedido {
    cliente_id: u32,
    produtos_ids: Vec<u32>,
}

fn main() {
    println!("Hello, world!");
    let (tx_clientes, rx_clientes) = mpsc::channel();
    let (tx_produtos, rx_produtos) = mpsc::channel();

    let clientes_thread = thread::spawn(move || {
        let clientes = vec![
            Cliente { id: 1, nome: "Alice".to_string() },
            Cliente { id: 2, nome: "Bob".to_string() },
        ];
        for cliente in clientes {
            println!("Enviando cliente: {}", cliente.nome);
            tx_clientes.send(cliente).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let produtos_thread = thread::spawn(move || {
        let produtos = vec![
            Produto { id: 1, nome: "Laptop".to_string() },
            Produto { id: 2, nome: "Smartphone".to_string() },
        ];
        for produto in produtos {
            println!("Enviando produto: {}", produto.nome);
            tx_produtos.send(produto).unwrap();
            thread::sleep(Duration::from_millis(700));
        }
    });

    clientes_thread.join().unwrap();
    produtos_thread.join().unwrap();

    let clientes: Vec<Cliente> = rx_clientes.try_iter().collect();
    let produtos: Vec<Produto> = rx_produtos.try_iter().collect();

    if !clientes.is_empty() || !produtos.is_empty() {
        let pedido = Pedido {
            cliente_id: clientes[0].id,
            produtos_ids: produtos.iter().map(|p| p.id).collect(),
        };
        println!("Pedido criado para cliente {} com produtos: {:?}", clientes[0].nome, produtos.iter().map(|p| &p.nome).collect::<Vec<&String>>());
    } else {
        println!("Nenhum cliente ou produto disponível para criar um pedido.");
    }

    println!("Processamento concluído.");
}
