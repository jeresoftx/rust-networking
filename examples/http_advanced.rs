use rust_networking::http::{HeaderMap, HttpRequest};

fn main() {
    let texto = concat!(
        "POST /lessons HTTP/1.1\r\n",
        "Host: jeresoft.test\r\n",
        "Content-Type: application/json\r\n",
        "\r\n",
        "{\"title\":\"HTTP\"}"
    );
    let solicitud = HttpRequest::parse(texto).unwrap();
    let cache = HeaderMap::new()
        .with_cache_control("max-age=60")
        .with_etag("\"lesson-http\"");

    println!("ruta: {}", solicitud.path());
    println!(
        "tipo de contenido: {}",
        solicitud.headers().get("Content-Type").unwrap()
    );
    println!("cuerpo: {}", String::from_utf8_lossy(solicitud.body()));
    println!("cache-control: {}", cache.cache_control().unwrap());
    println!("etag: {}", cache.etag().unwrap());
}
