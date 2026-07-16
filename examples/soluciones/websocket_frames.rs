use rust_networking::websocket::{Opcode, WebSocketFrame};

fn main() {
    let texto = WebSocketFrame::text("hola");
    let binario = WebSocketFrame::binary(vec![1, 2, 3]);

    assert_eq!(texto.opcode(), Opcode::Text);
    assert_eq!(texto.payload(), b"hola");
    assert_eq!(binario.opcode(), Opcode::Binary);
    println!("tramas creadas");
}
