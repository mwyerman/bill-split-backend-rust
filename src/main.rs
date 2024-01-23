use billsplit;

#[tokio::main]
pub async fn main() {
    let config = billsplit::config::Config::new();

    billsplit::start_server(config).await;

}

