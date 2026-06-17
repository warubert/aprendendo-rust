use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum ErroDeTransacao {
    SaldoInsuficiente { saldo_atual: f64, tentativa_saque: f64 },
    ErroDeAutenticacao,
    ErroDeRede,
    ErroComFonte {mensagem: String, fonte: Box<dyn Error>},
}

impl std::fmt::Display for ErroDeTransacao {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErroDeTransacao::SaldoInsuficiente { saldo_atual, tentativa_saque } => {
                write!(f, "Saldo insuficiente: saldo atual é {}, tentativa de saque é {}", saldo_atual, tentativa_saque)
            }
            ErroDeTransacao::ErroDeAutenticacao => write!(f, "Erro de autenticação"),
            ErroDeTransacao::ErroDeRede => write!(f, "Erro de rede"),
            ErroDeTransacao::ErroComFonte { mensagem, fonte } => write!(f, "{}: {}", mensagem, fonte),
        }
    }
}

impl std::error::Error for ErroDeTransacao {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErroDeTransacao::ErroComFonte { fonte, .. } => Some(fonte.as_ref()),
            _ => None,
        }
    }
}

fn processar_transacao(valor: f64, auth: bool, rede: bool, com_fonte: bool) -> Result<(), ErroDeTransacao> {
    let saldo = 1000.0; // Exemplo de saldo atual

    if valor > saldo {
        return Err(ErroDeTransacao::SaldoInsuficiente { saldo_atual: saldo, tentativa_saque: valor })
    }

    if !auth {
        return Err(ErroDeTransacao::ErroDeAutenticacao)
    }
    
    if !rede {
        return Err(ErroDeTransacao::ErroDeRede)
    }

    if com_fonte {
        let result_arquivo: Result<String, io::Error> = ler_conteudo_arquivo("arquivo_inexistente.txt");
        match result_arquivo {
            Ok(_) => (),
            Err(e) => return Err(ErroDeTransacao::ErroComFonte { mensagem: String::from("Erro ao ler arquivo"), fonte: Box::new(e) }),
        }
    }

    Ok(())
}

fn ler_conteudo_arquivo(nome_arquivo: &str) -> Result<String, io::Error> {
    let mut f: File = File::open(nome_arquivo)?;
    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}


fn main() {
    match processar_transacao(200.0, true, true, true) {
        Ok(_) => println!("Transação processada com sucesso!"),
        Err(e) => {
            println!("Erro ao processar transação: {}", e);
        
            if let Some(source) = e.source() {
                println!("Erro de origem: {}", source);
            }
        }
    }
}
