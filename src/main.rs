use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use backend::router::routes;
use backend::state::State;

#[macro_use]
extern crate serde_derive;
extern crate thiserror;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenv().ok();

    let pg_url = env::var("PGURL").expect("PGURL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await
        .expect("Failed to create pool");

    let state = State::new_db(db_pool);
    let app = routes().with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener,app).await?;
    Ok(())
}
