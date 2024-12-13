
use tokio::{ io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener};




#[tokio::main]
async fn main() {

    //tcp listener
    let listener = TcpListener::bind("localhost:3001").await.unwrap();

    //for multple clients

    
    loop {
        
            let(mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
        
        
                // let mut buffer = [0u8; 1024];
            
                // let bytes_read = socket.read(&mut buffer).await.unwrap();
                // socket.write_all(&buffer[..bytes_read]).await.unwrap();
                
                let(read, mut write) = socket.split();
                
                let mut reader = BufReader::new(read);
                let mut line = String::new();
        
        
                loop{
                    
                    let bytes_read = reader.read_line(&mut line).await.unwrap();
        
                    if bytes_read == 0 {
                        break;
                    }
                    write.write_all(&line.as_bytes()).await.unwrap();
                    line.clear();
                    
                }
            });
        }

}
