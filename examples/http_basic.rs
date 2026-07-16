use rust_networking::http::HttpRequest;

fn main() {
    let texto = "GET /academy HTTP/1.1\r\nHost: jeresoft.test\r\n\r\n";
    let solicitud = HttpRequest::parse(texto).unwrap();

    println!("método: {:?}", solicitud.method());
    println!("ruta: {}", solicitud.path());
    println!("host: {}", solicitud.headers().get("Host").unwrap());
}
