use rust_networking::udp::{UdpDatagram, UdpEndpoint};

fn main() {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);
    let datagrama =
        UdpDatagram::new(origen, destino, b"temperatura=31".to_vec()).expect("datagrama válido");

    println!(
        "origen: {}:{}",
        datagrama.source().address(),
        datagrama.source().port()
    );
    println!(
        "destino: {}:{}",
        datagrama.destination().address(),
        datagrama.destination().port()
    );
    println!("bytes de carga útil: {}", datagrama.len());
}
