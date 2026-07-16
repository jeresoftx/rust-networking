use std::hint::black_box;
use std::time::Instant;

use rust_networking::grpc::{GrpcMessage, GrpcMethod, GrpcService, StreamMode};

fn registrar_contratos(iteraciones: usize) {
    for _ in 0..iteraciones {
        let service = GrpcService::new("academy.LessonService", 1)
            .add_method(GrpcMethod::new("GetLesson", 1, StreamMode::Unary))
            .unwrap()
            .add_method(GrpcMethod::new(
                "WatchLessons",
                1,
                StreamMode::ServerStreaming,
            ))
            .unwrap();
        black_box(service);
    }
}

fn validar_compatibilidad(iteraciones: usize) {
    let service = GrpcService::new("academy.LessonService", 1)
        .add_method(GrpcMethod::new("GetLesson", 1, StreamMode::Unary))
        .unwrap();

    for _ in 0..iteraciones {
        let compatible = service.ensure_compatible("GetLesson", 1).is_ok();
        black_box(compatible);
    }
}

fn construir_mensajes(iteraciones: usize) {
    for _ in 0..iteraciones {
        let message = GrpcMessage::new("GetLessonRequest", br#"{"id":42}"#.to_vec());
        black_box(message);
    }
}

fn main() {
    let iteraciones = 30_000usize;

    let start = Instant::now();
    registrar_contratos(iteraciones);
    let contratos_elapsed = start.elapsed();

    let start = Instant::now();
    validar_compatibilidad(iteraciones);
    let compat_elapsed = start.elapsed();

    let start = Instant::now();
    construir_mensajes(iteraciones);
    let mensajes_elapsed = start.elapsed();

    println!("benchmark de grpc (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("registro de contratos: {contratos_elapsed:?}");
    println!("validación de compatibilidad: {compat_elapsed:?}");
    println!("construcción de mensajes: {mensajes_elapsed:?}");
}
