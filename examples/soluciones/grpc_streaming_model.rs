use rust_networking::grpc::{GrpcMethod, StreamMode};

fn main() {
    let method = GrpcMethod::new("WatchLessons", 1, StreamMode::ServerStreaming);

    assert!(method.mode().has_server_stream());
    assert!(!method.mode().has_client_stream());
    println!("modo de flujo del servidor");
}
