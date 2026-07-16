use rust_networking::grpc::StreamMode;

fn main() {
    for mode in [
        StreamMode::Unary,
        StreamMode::ServerStreaming,
        StreamMode::ClientStreaming,
        StreamMode::BidirectionalStreaming,
    ] {
        println!(
            "{mode:?}: cliente={} servidor={}",
            mode.has_client_stream(),
            mode.has_server_stream()
        );
    }
}
