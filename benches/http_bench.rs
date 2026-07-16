use std::hint::black_box;
use std::time::Instant;

use rust_networking::http::{HttpRequest, HttpResponse, StatusCode};

fn parsear_solicitudes(iteraciones: usize) {
    let texto = concat!(
        "GET /academy HTTP/1.1\r\n",
        "Host: jeresoft.test\r\n",
        "Accept: text/plain\r\n",
        "\r\n"
    );

    for _ in 0..iteraciones {
        let solicitud = HttpRequest::parse(texto).unwrap();
        black_box(solicitud);
    }
}

fn rechazar_metodos_invalidos(iteraciones: usize) {
    let texto = "FETCH /academy HTTP/1.1\r\nHost: jeresoft.test\r\n\r\n";

    for _ in 0..iteraciones {
        let error = HttpRequest::parse(texto).unwrap_err();
        black_box(error);
    }
}

fn construir_respuestas(iteraciones: usize) {
    for _ in 0..iteraciones {
        let respuesta = HttpResponse::new(StatusCode::Ok)
            .with_header("Content-Type", "text/plain")
            .with_header("Cache-Control", "max-age=60")
            .with_body(b"hola".to_vec());
        black_box(respuesta);
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    parsear_solicitudes(iteraciones);
    let parseo_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_metodos_invalidos(iteraciones);
    let rechazo_elapsed = start.elapsed();

    let start = Instant::now();
    construir_respuestas(iteraciones);
    let respuesta_elapsed = start.elapsed();

    println!("benchmark de http (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("parseo de solicitud GET: {parseo_elapsed:?}");
    println!("rechazo de método inválido: {rechazo_elapsed:?}");
    println!("construcción de respuesta: {respuesta_elapsed:?}");
}
