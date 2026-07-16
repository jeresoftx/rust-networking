use rust_networking::grpc::{GrpcMessage, GrpcMethod, GrpcService, GrpcStatus, StreamMode};

fn main() {
    let service = GrpcService::new("academy.RecommendationService", 2)
        .add_method(GrpcMethod::new(
            "WatchRecommendations",
            2,
            StreamMode::ServerStreaming,
        ))
        .unwrap();
    service
        .ensure_compatible("WatchRecommendations", 2)
        .unwrap();

    let request = GrpcMessage::new("RecommendationRequest", br#"{"student_id":42}"#.to_vec());
    let status = GrpcStatus::ok();

    println!("servicio: {}", service.name());
    println!("mensaje: {}", request.type_name());
    println!("estado: {} {}", status.code(), status.name());
}
