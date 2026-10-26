#[cfg(test)]
mod tests {
    use tokio::{io::AsyncWriteExt, net::{TcpStream, UdpSocket}};
    use crate::kernel::{PayloadAccelerometer, PayloadTachometer};

    #[tokio::test]
    async fn send_to_udp() {
        let udp_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
        let udp_target = "localhost:8081";

        let tachometer_data = PayloadTachometer {
            id: "tach-sensor-01".to_string(),
            work_time: 1245.8,
            rpm: 2980.5,
        };
        let udp_json = serde_json::to_vec(&tachometer_data).unwrap();
        udp_socket.send_to(&udp_json, udp_target).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    #[tokio::test]
    async fn send_to_tcp() {
        let tcp_target = "localhost:8080";
        let mut tcp_stream = TcpStream::connect(tcp_target).await.unwrap();

        let accelerometer_data = PayloadAccelerometer {
            id: "accel-sensor-99".to_string(),
            work_time: 1245.8,
            buf: vec![0.01, 0.05, 0.12, 0.45, 0.89, 0.32, -0.11, -0.54],
        };

        let tcp_json = serde_json::to_vec(&accelerometer_data).unwrap();
        let _ = tcp_stream.write_all(&tcp_json).await;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}