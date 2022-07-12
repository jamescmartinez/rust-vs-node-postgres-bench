use chrono::Utc;
use tokio_postgres::{Error, NoTls};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    let start = Utc::now();

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=bench", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    for _ in std::iter::repeat(()).take(100000) {
        client
            .query("insert into bench default values", &[])
            .await?;
    }

    println!("{}", Utc::now() - start);

    Ok(())
}
