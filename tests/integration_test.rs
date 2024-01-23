// Integration tests for API endpoints
use tokio::task;
use tokio;
// use axum::Server;

fn start_server() -> task::JoinHandle<()> {
    task::spawn(async {
        let config = billsplit::config::Config::new();
        billsplit::start_server(config).await;
    })
}

fn stop_server(server: task::JoinHandle<()>) {
    server.abort();
}

#[tokio::test]
async fn test_get_bills() {
    let server = start_server();
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/bills")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    stop_server(server);
}
