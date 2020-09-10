use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:5000").await?;

    let (tx, _rx) = broadcast::channel(16);

    let tx = Arc::new(tx);

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Connected");
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let (mut rd, mut wr) = io::split(socket);

        tokio::spawn(async move {
            loop {
                let mut buffer = vec![0; 1024];

                match rd.read(&mut buffer).await {
                    Ok(0) => return,
                    Ok(_n) => {
                        let message = String::from_utf8(buffer).unwrap();
                        println!("Recieved: {}", message);
                        tx.send(message).unwrap();
                    }
                    Err(_) => {
                        println!("Reading from socket failed");
                        return;
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                let message = rx.recv().await.unwrap();
                println!("Sending message");
                wr.write_all(message.as_bytes()).await.unwrap();
            }
        });
    }
}
