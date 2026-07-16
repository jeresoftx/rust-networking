use rust_networking::grpc::GrpcStatus;

fn main() {
    let status = GrpcStatus::not_found("lección no encontrada");

    assert_eq!(status.code(), 5);
    assert_eq!(status.name(), "NOT_FOUND");
    println!("estado: {} {}", status.code(), status.name());
}
