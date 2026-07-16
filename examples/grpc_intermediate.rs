use rust_networking::grpc::GrpcStatus;

fn main() {
    let ok = GrpcStatus::ok();
    let not_found = GrpcStatus::not_found("lección no encontrada");

    println!("{} {}", ok.code(), ok.name());
    println!(
        "{} {} {}",
        not_found.code(),
        not_found.name(),
        not_found.message()
    );
}
