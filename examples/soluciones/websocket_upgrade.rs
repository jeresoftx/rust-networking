use rust_networking::http::HttpRequest;
use rust_networking::websocket::{WebSocketState, WebSocketUpgrade};

fn main() {
    let solicitud = HttpRequest::parse(
        "GET /notificaciones HTTP/1.1\r\n\
         Host: academy.jeresoft.test\r\n\
         Connection: Upgrade\r\n\
         Upgrade: websocket\r\n\
         Sec-WebSocket-Key: clave-educativa\r\n\r\n",
    )
    .unwrap();
    let conexion = WebSocketUpgrade::new(solicitud).accept().unwrap();

    assert_eq!(conexion.state(), WebSocketState::Open);
    println!("actualización aceptada");
}
