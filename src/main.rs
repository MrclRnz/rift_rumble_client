mod lcu_ws;

#[tokio::main]
async fn main() {
    tokio::spawn(
        lcu_ws::get_lcu_websocket_connection()
        ).await;
}
