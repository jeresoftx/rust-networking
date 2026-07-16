use rust_networking::http::{HttpMethod, HttpRequest, HttpResponse, StatusCode};

fn responder(texto: &str) -> HttpResponse {
    match HttpRequest::parse(texto) {
        Ok(solicitud)
            if solicitud.method() == HttpMethod::Get && solicitud.path() == "/academy/status" =>
        {
            HttpResponse::new(StatusCode::Ok)
                .with_header("Content-Type", "text/plain")
                .with_header("Cache-Control", "max-age=30")
                .with_body(b"curso disponible".to_vec())
        }
        Ok(_) => HttpResponse::new(StatusCode::NotFound)
            .with_header("Content-Type", "text/plain")
            .with_body(b"ruta inexistente".to_vec()),
        Err(_) => HttpResponse::new(StatusCode::BadRequest)
            .with_header("Content-Type", "text/plain")
            .with_body("solicitud inválida".as_bytes().to_vec()),
    }
}

fn main() {
    let respuesta = responder("GET /academy/status HTTP/1.1\r\nHost: jeresoft.test\r\n\r\n");

    println!(
        "estado: {} {}",
        respuesta.status().code(),
        respuesta.reason_phrase()
    );
    println!("cuerpo: {}", String::from_utf8_lossy(respuesta.body()));
}
