use tokio_postgres::{Client, NoTls};

#[tokio::main]
async fn main() {
    let mut client = connect().await;
    let mut tran = client.transaction().await.expect("transaction");

    loop {
        tran.commit().await.expect("committed");
        tran = client.transaction().await.expect("second transaction");
    }
}

async fn connect() -> Client {
    let (client, connection) = tokio_postgres::connect("", NoTls).await.expect("connect");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}
