use std::hint::black_box;
use std::time::Instant;

use rust_networking::http::HttpRequest;
use rust_networking::websocket::{CloseCode, WebSocketFrame, WebSocketUpgrade};

fn solicitud_upgrade() -> HttpRequest {
    HttpRequest::parse(
        "GET /notificaciones HTTP/1.1\r\n\
         Host: academy.jeresoft.test\r\n\
         Connection: Upgrade\r\n\
         Upgrade: websocket\r\n\
         Sec-WebSocket-Key: clave-educativa\r\n\r\n",
    )
    .unwrap()
}

fn validar_upgrade(iteraciones: usize) {
    for _ in 0..iteraciones {
        let conexion = WebSocketUpgrade::new(solicitud_upgrade()).accept().unwrap();
        black_box(conexion);
    }
}

fn crear_tramas(iteraciones: usize) {
    for _ in 0..iteraciones {
        black_box(WebSocketFrame::text("notificación"));
        black_box(WebSocketFrame::binary(vec![1, 2, 3, 4]));
        black_box(WebSocketFrame::close(CloseCode::NormalClosure));
    }
}

fn ping_pong(iteraciones: usize) {
    for _ in 0..iteraciones {
        let ping = WebSocketFrame::ping(b"latido".to_vec());
        let pong = ping.respond_to_ping().unwrap();
        black_box(pong);
    }
}

fn main() {
    let iteraciones = 30_000usize;

    let start = Instant::now();
    validar_upgrade(iteraciones);
    let upgrade_elapsed = start.elapsed();

    let start = Instant::now();
    crear_tramas(iteraciones);
    let frames_elapsed = start.elapsed();

    let start = Instant::now();
    ping_pong(iteraciones);
    let ping_elapsed = start.elapsed();

    println!("benchmark de websocket (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("validación de actualización: {upgrade_elapsed:?}");
    println!("creación de tramas: {frames_elapsed:?}");
    println!("ping/pong: {ping_elapsed:?}");
}
