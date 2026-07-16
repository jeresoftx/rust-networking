use rust_networking::http::{HttpResponse, StatusCode};

fn main() {
    let respuesta = HttpResponse::new(StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_body(b"hola".to_vec());

    assert_eq!(respuesta.status().code(), 200);
    assert_eq!(respuesta.body(), b"hola");
    println!(
        "respuesta: {} {}",
        respuesta.status().code(),
        respuesta.reason_phrase()
    );
}
