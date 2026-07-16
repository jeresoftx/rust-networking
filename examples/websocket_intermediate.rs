use rust_networking::websocket::WebSocketFrame;

fn main() {
    let texto = WebSocketFrame::text("hola");
    let binario = WebSocketFrame::binary(vec![1, 2, 3]);

    println!("texto: {:?} {:?}", texto.opcode(), texto.payload());
    println!("binario: {:?} {:?}", binario.opcode(), binario.payload());
}
