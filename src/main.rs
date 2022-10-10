mod db;

use sqlx::postgres::PgPoolOptions;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 2 {
        println!("<corpus> <limit>");
        std::process::exit(0);
    }

    let corpus = args[0].clone();
    let limit = args[1]
        .parse::<i64>()
        .expect("second argument to be positive integer");

    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db::client_str())
        .await?;

    let query = format!(
        "SELECT tokenized
            FROM {corpus}
            ORDER BY tokenized
            LIMIT {limit}"
    );
    let query = sqlx::query_as::<_, (String,)>(&query);

    let mut stream = query.fetch(&pool);

    while let Some(row) = stream.try_next().await? {
            dbg!(row);
    }

    // let row: (String,) = query.fetch_one(&pool).await?;
    // dbg!(&row);

    Ok(())
}
