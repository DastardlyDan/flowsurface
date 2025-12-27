use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde_json::from_str;
use crate::data_format::ChartData;

pub async fn connect_to_server(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to server at {}", url);

    let (mut write, mut read) = ws_stream.split();

    // Example: Sending a subscription message
    let subscription_message = Message::Text("{\"action\":\"subscribe\",\"symbol\":\"AAPL\"}".to_string());
    write.send(subscription_message).await?;

    // Reading messages from the server
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match from_str::<ChartData>(&text) {
                    Ok(chart_data) => {
                        println!("Received chart data: {:?}", chart_data);
                        // Process the chart data here
                    }
                    Err(e) => {
                        eprintln!("Failed to parse chart data: {}", e);
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
            }
        }
    }

    Ok(())
}