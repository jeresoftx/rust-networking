use rust_networking::grpc::{GrpcMethod, GrpcService, StreamMode};

fn main() {
    let service = GrpcService::new("academy.LessonService", 1)
        .add_method(GrpcMethod::new("GetLesson", 1, StreamMode::Unary))
        .unwrap();

    assert_eq!(service.method("GetLesson").unwrap().name(), "GetLesson");
    println!("contrato registrado");
}
