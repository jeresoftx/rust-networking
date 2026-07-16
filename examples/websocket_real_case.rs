use rust_networking::http::HttpRequest;
use rust_networking::websocket::{CloseCode, WebSocketFrame, WebSocketUpgrade};

fn main() {
    let solicitud = HttpRequest::parse(
        "GET /panel HTTP/1.1\r\n\
         Host: academy.jeresoft.test\r\n\
         Connection: Upgrade\r\n\
         Upgrade: websocket\r\n\
         Sec-WebSocket-Key: panel\r\n\r\n",
    )
    .unwrap();
    let mut conexion = WebSocketUpgrade::new(solicitud).accept().unwrap();

    conexion
        .apply_frame(WebSocketFrame::text("notificación: nueva lección"))
        .unwrap();
    let respuesta = conexion
        .apply_frame(WebSocketFrame::ping(b"vivo".to_vec()))
        .unwrap()
        .unwrap();

    println!("tramas recibidas: {}", conexion.received().len());
    println!("respuesta control: {:?}", respuesta.opcode());

    conexion
        .apply_frame(WebSocketFrame::close(CloseCode::NormalClosure))
        .unwrap();
    println!("estado final: {:?}", conexion.state());
}
