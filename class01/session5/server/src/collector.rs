use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS, decode_v1};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

pub async fn data_collector(connection: Pool<Sqlite>) -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address.
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections.
    loop {
        // Wait for a new connection.
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address, connection.clone()));
    }
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr, connection: Pool<Sqlite>) {
    println!("New connection from {:?}", address);
    let mut buf = vec![0_u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("Failed to read data from socket.");

        if n == 0 {
            println!("No data received - connection closed.");
            return;
        }

        let received_data = decode_v1(&buf[..n]);

        match received_data {
            (
                timestamp,
                CollectorCommandV1::SubmitData {
                    collector_id,
                    total_memory,
                    used_memory,
                    average_cpu_usage,
                },
            ) => {
                let collector_id = uuid::Uuid::from_u128(collector_id).to_string();

                let result = sqlx::query(
                    r#"
					INSERT INTO timeseries (collector_id, received, total_memory, used_memory, average_cpu)
					VALUES (?, ?, ?, ?, ?)
					"#,
                )
                .bind(collector_id)
                .bind(timestamp)
                .bind(total_memory as i64)
                .bind(used_memory as i64)
                .bind(average_cpu_usage)
                .execute(&connection)
                .await;

                if result.is_err() {
                    println!("Error inserting data into the database: {:?}", result);
                }
            }
        };
    }
}
