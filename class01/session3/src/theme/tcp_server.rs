// IKinder

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{io, net, time};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    crate::show_name(file!());

    tokio::spawn(client_runner());

    let listener = net::TcpListener::bind("127.0.0.1:8123").await?;
    loop {
        let (mut socket, address) = listener.accept().await?;
        tokio::spawn(async move {
            println!("Connection from {:?}", address);
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                if n == 0 {
                    return;
                }
                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}

async fn client_runner() {
    loop {
        time::sleep(time::Duration::from_secs(1)).await;
        let _ = tcp_client().await;
    }
}

async fn tcp_client() -> anyhow::Result<()> {
    let mut stream = net::TcpStream::connect("127.0.0.1:8123").await?;
    println!("Connected to the server");

    stream.write_all(b"Hello World").await?;
    let mut buf = vec![0u8; 1024];
    let bytes_read = stream.read(&mut buf).await?;
    println!("Response: {}", String::from_utf8_lossy(&buf[0..bytes_read]));

    Ok(())
}
