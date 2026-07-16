use rust_networking::http::HttpRequest;

fn main() {
    let solicitud =
        HttpRequest::parse("GET /academy HTTP/1.1\r\nHost: jeresoft.test\r\n\r\n").unwrap();

    assert_eq!(solicitud.path(), "/academy");
    println!("ruta parseada: {}", solicitud.path());
}
