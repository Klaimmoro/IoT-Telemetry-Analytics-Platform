use std::sync::Arc;
use axum::{Extension, Json, Router, routing::post};
use rdkafka::producer::FutureProducer;
use tokio::{io::AsyncReadExt, net::UdpSocket, net::TcpListener};
use kernel::{
    PayloadAccelerometer, 
    PayloadTachometer, 
    send_to_kafka, 
    create_kafka_producer
};

struct AppState {
    producer: Arc<FutureProducer>,
    tachometer_topic: &'static str, 
    accelerometer_topic: &'static str,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Ingestion Service...");

    let kafka_brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    
    // ИСПРАВЛЕНО: Вызываем функцию, которая пришла к нам из библиотеки kernel
    let producer = Arc::new(create_kafka_producer(&kafka_brokers));

    let tcp_producer = Arc::clone(&producer);
    let udp_producer = Arc::clone(&producer);

    let tachometer_topic = "tachometer-raw-data";
    let accelerometer_topic = "accelerometer-raw-data";
    


    let udp_task = async move {
        let addr = std::env::var("INGESTION_UDP_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".to_string());
        let socket = UdpSocket::bind(&addr).await.expect("Failed to bind UDP socket");
        tracing::info!("UDP server listening on {}", addr);
        
        let mut buf = vec![0u8; 1024];
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, _remote_addr)) => {
                    let data = buf[..len].to_vec();
                    let p = Arc::clone(&udp_producer);
                    tokio::spawn(async move {
                        process_udp(data, p, tachometer_topic).await;
                    });
                },
                Err(e) => tracing::error!("UDP read error: {}", e),
            }
        }
    };

    let shared_state = Arc::new(
        AppState { 
            producer: Arc::clone(&producer), 
            tachometer_topic, 
            accelerometer_topic 
        }
    );

    let app = Router::new()
        .route("/telemetry/accelerometer", post(handle_accelerometer))
        .layer(Extension(shared_state));

    let addr = std::env::var("INGESTION_TCP_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind TCP socket");

    tokio::select! {
        _ = udp_task => tracing::info!("UDP task finished"),
        res = axum::serve(listener, app) => {
            if let Err(e) = res {
                tracing::error!("Axum server error: {}", e);
            }
        },
    }
}

async fn process_udp(data: Vec<u8>, producer: Arc<FutureProducer>, topic: &str) {
    send_to_kafka::<PayloadTachometer>(data, producer, topic).await;
}


async fn handle_accelerometer(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<PayloadAccelerometer>,
) -> &'static str {
    tracing::debug!("Axum recieved accelerometer data from: {}", payload.id);
    if let Ok(data) = serde_json::to_vec(&payload) {
        let p = Arc::clone(&state.producer);
        let topic = state.accelerometer_topic;

        tokio::spawn(async move {
            send_to_kafka::<PayloadAccelerometer>(data, p, topic).await;
        });
    }
    "OK"
}