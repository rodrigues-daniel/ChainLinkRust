use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // Conecta ao servidor TCP
    let mut stream = match TcpStream::connect("127.0.0.1:8080").await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Erro ao conectar ao servidor: {}", e);
            return;
        }
    };

    // Envia uma mensagem para o servidor
    let message = "Olá servidor! Esta é uma mensagem do cliente.";
    if let Err(e) = stream.write_all(message.as_bytes()).await {
        eprintln!("Erro ao enviar a mensagem para o servidor: {}", e);
        return;
    }

    // Lê a resposta do servidor
    let mut buffer = [0; 1024];
    let n = match stream.read(&mut buffer).await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Erro ao ler a resposta do servidor: {}", e);
            return;
        }
    };

    // Converte os dados em uma string e exibe no console
    let received_message = String::from_utf8_lossy(&buffer[..n]);
    println!("Resposta do servidor: {}", received_message);
}
