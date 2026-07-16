use rust_networking::grpc::{GrpcMethod, GrpcService, StreamMode};

fn main() {
    let service = GrpcService::new("academy.LessonService", 1)
        .add_method(GrpcMethod::new("GetLesson", 1, StreamMode::Unary))
        .unwrap();

    let method = service.method("GetLesson").unwrap();
    println!("servicio: {}", service.name());
    println!("método: {} {:?}", method.name(), method.mode());
}
