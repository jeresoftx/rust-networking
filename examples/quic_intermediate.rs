use rust_networking::quic::{QuicConnectionId, QuicPacket, QuicStream, QuicStreamId};

fn main() {
    let connection = QuicConnectionId::new("cid-streams").unwrap();
    let control_id = QuicStreamId::new(0);
    let datos_id = QuicStreamId::new(4);
    let mut control = QuicStream::new(control_id);
    let mut datos = QuicStream::new(datos_id);

    control
        .receive(QuicPacket::new(
            connection.clone(),
            control_id,
            1,
            "abrir sesión".as_bytes().to_vec(),
        ))
        .unwrap();
    datos
        .receive(QuicPacket::new(
            connection,
            datos_id,
            1,
            b"bloque de datos".to_vec(),
        ))
        .unwrap();

    println!("control: {}", control.payloads_in_order().len());
    println!("datos: {}", datos.payloads_in_order().len());
}
