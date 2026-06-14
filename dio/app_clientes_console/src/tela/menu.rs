use crate::tela::{ler::*, operacoes_basicas::*};
use crate::models::cliente::Cliente;
use crate::tela::servico_cliente::*;

pub fn mostrar_menu(clientes: &mut Vec<Cliente>) {
    
    loop {
        limpar_tela();
        println!("\
            ============ MENU =============== \n\
            Escolha uma das opções abaixo: \n\
            1 - Cadastrar cliente \n\
            2 - Alterar cliente \n\
            3 - Excluir cliente \n\
            4 - Listar clientes \n\
            0 - Sair
        ");

        let opcao = ler_dados_int();

        match opcao {
            1 => incluir_cliente(clientes),
            2 => alterar_cliente(clientes),
            3 => excluir_cliente(clientes),
            4 => listar_clientes(clientes),
            0 => {
                println!("Saindo...");
                break;
            },
            _ => println!("Opção inválida"),
        }
    }

}