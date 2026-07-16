use rust_networking::udp::{DeliveryOutcome, UdpDatagram, UdpEndpoint};

fn main() {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);
    let datagrama =
        UdpDatagram::new(origen, destino, b"temperatura=31".to_vec()).expect("datagrama válido");

    assert_eq!(
        DeliveryOutcome::deterministic(datagrama, 5),
        DeliveryOutcome::Lost
    );
}
