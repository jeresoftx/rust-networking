use rust_networking::websocket::WebSocketFrame;

fn main() {
    let ping = WebSocketFrame::ping(b"latido".to_vec());
    let pong = ping.respond_to_ping().unwrap();

    println!("ping: {:?} {:?}", ping.opcode(), ping.payload());
    println!("pong: {:?} {:?}", pong.opcode(), pong.payload());
}
