use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on port 8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        println!("Cliente conectado: {}", socket.peer_addr()?);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            
            loop {
                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("Cliente encerrou conexao");
                        return;
                    },
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            println!("Erro ao enviar resposta");
                            return;
                        }
                    },
                    Err(e) => {
                        println!("Erro ao ler do socket: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
// use std::net::TcpListener;
// use std::io::{self, Read, Write};

// fn main() -> io::Result<()> {
//     println!("Hello, world!");

//     let listener = TcpListener::bind("127.0.0.1:8080")?;

//     println!("Server listening on port 8080");

//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut stream) => {
//                 println!("Cliente conectado: {}", stream.peer_addr()?);

//                 let mut buffer = [0; 1024];
//                 let bytes_read = stream.read(&mut buffer)?;

//                 if bytes_read > 0 {
//                     let message = String::from_utf8_lossy(&buffer[..bytes_read]);
//                     println!("Mensagem recebida: {}", message);

//                     let response = "Mensagem recebida com sucesso \n";
//                     stream.write_all(response.as_bytes())?;
//                 }

//                 println!("Cliente desconectado");
//             }
//             Err(e) => {
//                 eprintln!("Erro ao aceitar conexão: {}", e);
//             }
//         }
//     }

//     Ok(())
// }
