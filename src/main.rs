mod db;

use sqlx::postgres::PgPoolOptions;
use tokio_stream::StreamExt;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Config {
    Query {
        #[structopt()]
        query: String,
    },
    Corpus {
        #[structopt()]
        corpus: String,
        #[structopt()]
        limit: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let config = Config::from_args();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db::client_str())
        .await?;

    let query = match config {
        Config::Corpus { corpus, limit } => format!(
            "SELECT tokenized
            FROM {corpus}
            ORDER BY tokenized
            LIMIT {limit}"
        ),
        Config::Query { query } => query,
    };

    let query = sqlx::query_as::<_, (String,)>(&query);

    let mut stream = query.fetch(&pool);

    while let Some(row) = stream.try_next().await? {
        dbg!(row);
    }

    // let row: (String,) = query.fetch_one(&pool).await?;
    // dbg!(&row);

    Ok(())
}
