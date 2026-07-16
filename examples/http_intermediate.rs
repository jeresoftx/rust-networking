use rust_networking::http::{HttpResponse, StatusCode};

fn main() {
    let respuesta = HttpResponse::new(StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_body(b"hola desde HTTP".to_vec());

    println!(
        "estado: {} {}",
        respuesta.status().code(),
        respuesta.reason_phrase()
    );
    println!("tipo: {}", respuesta.headers().get("content-type").unwrap());
    println!("cuerpo: {}", String::from_utf8_lossy(respuesta.body()));
}
