use crate::{models::cliente::Cliente, tela::{ler::{ler_dados, ler_dados_int}, operacoes_basicas::*}};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    let mut cliente = Cliente::default();
    cliente.id = clientes.len() + 1;

    digitar_dados_cliente(&mut cliente);

    clientes.push(cliente);
    limpar_tela();
    println!("Cliente cadastrado com sucesso!");
    esperar(1);
}

fn digitar_dados_cliente(cliente: &mut Cliente) {
    println!("Digite o nome do cliente: ");
    cliente.nome = ler_dados();
    println!("Digite o CPF do cliente: ");
    cliente.cpf = ler_dados();
    println!("Digite o endereço do cliente: ");
    cliente.endereco = ler_dados();
}

pub fn alterar_cliente(clientes: &mut [Cliente]) {
    limpar_tela();

    if nao_tem_clientes(clientes) {
        return;
    }

    let id = captura_id();
    if let Some((i, cliente)) = buscar_cliente_por_id(clientes, id) {
        println!("{}", "-".to_string().repeat(40));
        println!("Alterando o cliente");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        digitar_dados_cliente(&mut clientes[i]);
        limpar_tela();
        println!("Cliente alterado com sucesso!");
    } else {
        limpar_tela();
        println!("Cliente com ID {} não encontrado!", id);
    }
    
    esperar(1);
}

pub fn excluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    if nao_tem_clientes(clientes) {
        return;
    }

    let id = captura_id();
    if let Some((i, cliente)) = buscar_cliente_por_id(clientes, id) {
        println!("{}", "-".to_string().repeat(40));
        println!("Confirma a exclusão do cliente abaixo? (s/n)");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        println!("\
            s - Sim, excluir o cliente \n\
            n - Não, cancelar a exclusão
        ");
        let opcao = ler_dados();
        if opcao.to_lowercase() == "s" {
            clientes.remove(i);
            limpar_tela();
            println!("Cliente excluído com sucesso!");
        } else {
            limpar_tela();
            println!("Exclusão cancelada!");
        }
        digitar_dados_cliente(&mut clientes[i]);
        limpar_tela();
        println!("Cliente alterado com sucesso!");
    } else {
        limpar_tela();
        println!("Cliente com ID {} não encontrado!", id);
    }
    
    esperar(1);
}

fn buscar_cliente_por_id(clientes: &[Cliente], id: usize) -> Option<(usize, &Cliente)>{
    clientes.iter().enumerate().find(|(_, cliente)| cliente.id == id)
}

fn captura_id() -> usize {
    limpar_tela();
    println!("Digite o ID do cliente que deseja alterar: ");
    ler_dados_int()
}

pub fn listar_clientes(clientes: &Vec<Cliente>) {
    limpar_tela();

    if nao_tem_clientes(clientes) {
        return;
    }

    println!("{}", "-".to_string().repeat(40));

    for cliente in clientes {
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
    }
    
    println!("Pressione Enter para continuar...");
    ler_dados();
}

fn nao_tem_clientes(clientes: &[Cliente]) -> bool {
    if clientes.is_empty() {
        println!("Não existem clientes cadastrados!");
        esperar(1);
        return true;
    }

    false
}

fn mostrar_cliente(cliente: &Cliente) {
    println!("ID: {}", cliente.id);
    println!("Nome: {}", cliente.nome);
    println!("CPF: {}", cliente.cpf);
    println!("Endereço: {}", cliente.endereco);
}