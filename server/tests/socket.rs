use std::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};

use cyclonegame_server::startup::run;

pub struct TestApp {
    pub address: String
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0") .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("127.0.0.1:{}", port);

    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address
    }
}

#[tokio::test]
async fn test_websocket_connection() {
    let app = spawn_app().await;
    let (mut ws_stream, response) = connect_async(format!("ws://{}/ws", app.address)).await.expect("Failed to connect");

    let status_code = response.status().as_u16();
    
    assert!((100..=299).contains(&status_code), "Unexpected status code: {}", status_code);

    let sent_msg = "Hello WebSocket";
    ws_stream.send(Message::Text(sent_msg.into())).await.expect("Failed to send message");

    let response_msg = ws_stream
        .next()
        .await
        .expect("Failed to receive a response")
        .expect("Failed to receive a message");
    
    if let Message::Text(received_msg) = response_msg {
        assert_eq!(received_msg, sent_msg, "Messages do not match");
    } else {
        panic!("Received msg is not text")
    }
}
