use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};




#[tokio::main]
async fn main() {

    //tcp listener
    let listener = TcpListener::bind("localhost:3001").await.unwrap();
    let(mut socket, _) = listener.accept().await.unwrap();

    loop {

        let mut buffer = [0u8; 1024];
    
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
