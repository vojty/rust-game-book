use io::AsyncReadExt;
use std::io::BufRead;
use std::process;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:5000").await?;
    let stdin = std::io::stdin();
    let (mut rd, mut wr) = io::split(stream);

    // thread for recieving messages from the server
    let handle = tokio::spawn(async move {
        loop {
            let mut buf = vec![0; 1024];
            match rd.read(&mut buf).await {
                Ok(0) => {
                    // Shutdown here
                    process::exit(0);
                }
                Ok(_n) => {
                    println!("<- Recieved message\n{}", String::from_utf8(buf).unwrap());
                }
                Err(_) => {
                    println!("Sent error");
                }
            }
        }
    });

    // Loop for reading the input
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("-> Sending {} chars", line.len());
        wr.write_all(line.as_bytes()).await.unwrap();
    }

    handle.await?;

    Ok(())
}
