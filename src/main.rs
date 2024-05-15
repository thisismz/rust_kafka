mod error;
mod kafka;
mod router;
mod state;
use router::init_router;
use state::AppState;

#[tokio::main]
async fn main() {
    let con = kafka::create_kafka_consumer();
    tokio::spawn(async move {
        kafka::kafka_consumer_task(con).await;
    });
    let state = AppState::new();

    let app = init_router(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
