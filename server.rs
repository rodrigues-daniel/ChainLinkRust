use tokio::net::TcpListener;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    // Inicia o servidor TCP na porta 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Servidor escutando em 127.0.0.1:8080");

    // Aceita e manipula conexões de clientes
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(stream) => {
                // Manipula a conexão em uma nova tarefa (thread)
                tokio::spawn(async move {
                    handle_client(stream).await;
                });
            }
            Err(e) => {
                eprintln!("Erro ao aceitar a conexão: {}", e);
            }
        }
    }
}

async fn handle_client(mut stream: tokio::net::TcpStream) {
    // Lê os dados do cliente
    let mut buffer = [0; 1024];
    let n = match stream.read(&mut buffer).await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Erro ao ler os dados do cliente: {}", e);
            return;
        }
    };

    // Converte os dados em uma string e exibe no console
    let received_message = String::from_utf8_lossy(&buffer[..n]);
    println!("Mensagem recebida: {}", received_message);

    // Envia uma mensagem de resposta para o cliente
    let response = "Olá cliente! Recebi sua mensagem.".as_bytes();
    if let Err(e) = stream.write_all(response).await {
        eprintln!("Erro ao enviar a resposta para o cliente: {}", e);
    }
}
