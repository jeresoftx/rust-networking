use rust_networking::udp::{DeliveryOutcome, UdpDatagram, UdpEndpoint};

fn main() {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);
    let datagrama =
        UdpDatagram::new(origen, destino, b"temperatura=31".to_vec()).expect("datagrama válido");

    for secuencia in 1..=5 {
        let resultado = DeliveryOutcome::deterministic(datagrama.clone(), secuencia);
        println!("secuencia {secuencia}: {resultado:?}");
    }
}
