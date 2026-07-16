use rust_networking::udp::{UdpDatagram, UdpEndpoint, UdpError};

fn main() {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);
    let carga = vec![0; UdpDatagram::MAX_PAYLOAD_SIZE + 1];

    let error = UdpDatagram::new(origen, destino, carga).unwrap_err();

    assert_eq!(
        error,
        UdpError::PayloadTooLarge {
            size: UdpDatagram::MAX_PAYLOAD_SIZE + 1,
            max: UdpDatagram::MAX_PAYLOAD_SIZE,
        }
    );
}
