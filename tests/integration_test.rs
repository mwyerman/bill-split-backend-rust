// Integration tests for API endpoints
use tokio::task;
use tokio;
use uuid::Uuid;
use billsplit::models::bill::{Bill, BillWithId};
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
    let body = response.text().await.unwrap();
    let bills: Vec<BillWithId> = serde_json::from_str(&body).unwrap();
    assert_eq!(bills.len(), 0);

    let new_bill = Bill::new("test".to_string());
    let response = client
        .post("http://localhost:3000/bill/insert")
        .json(&new_bill)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let body = &body[1..body.len() - 1];
    let uuid = Uuid::parse_str(&body).unwrap();

    let response = client
        .get("http://localhost:3000/bill/".to_string() + &uuid.to_string())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let bill: Bill = serde_json::from_str(&body).unwrap();
    assert_eq!(bill, new_bill);

    let response = client
        .get("http://localhost:3000/bills")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    // parse response body (json)
    let body = response.text().await.unwrap();
    let bills: Vec<BillWithId> = serde_json::from_str(&body).unwrap();
    assert_eq!(bills.len(), 1);
    assert_eq!(bills[0].bill, new_bill);

    // delete
    let response = client
        .delete("http://localhost:3000/bill/".to_string() + &uuid.to_string())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let body = &body[1..body.len() - 1];
    let uuid = Uuid::parse_str(&body).unwrap();

    let response = client
        .get("http://localhost:3000/bill/".to_string() + &uuid.to_string())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 404);

    let response = client
        .get("http://localhost:3000/bills")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    // parse response body (json)
    let body = response.text().await.unwrap();
    let bills: Vec<BillWithId> = serde_json::from_str(&body).unwrap();
    assert_eq!(bills.len(), 0);

    let response = client
        .post("http://localhost:3000/bill/new")
        .json("test")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let body = &body[1..body.len() - 1];
    let uuid = Uuid::parse_str(&body).unwrap();

    let response = client
        .get("http://localhost:3000/bill/".to_string() + &uuid.to_string())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let bill: Bill = serde_json::from_str(&body).unwrap();
    assert_eq!(bill.name, "test");

    stop_server(server);
}
