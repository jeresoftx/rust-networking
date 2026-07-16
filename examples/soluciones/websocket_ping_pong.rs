use rust_networking::websocket::{Opcode, WebSocketFrame};

fn main() {
    let ping = WebSocketFrame::ping(b"latido".to_vec());
    let pong = ping.respond_to_ping().unwrap();

    assert_eq!(pong.opcode(), Opcode::Pong);
    assert_eq!(pong.payload(), b"latido");
    println!("pong generado");
}
