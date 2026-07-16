use rust_networking::udp::{DeliveryOutcome, UdpDatagram, UdpEndpoint, UdpError};

#[test]
fn datagram_records_source_destination_and_payload() {
    let source = UdpEndpoint::new("10.0.0.10", 50_000);
    let destination = UdpEndpoint::new("10.0.0.20", 8125);

    let datagram = UdpDatagram::new(source.clone(), destination.clone(), b"cpu=42".to_vec())
        .expect("datagrama válido");

    assert_eq!(datagram.source(), &source);
    assert_eq!(datagram.destination(), &destination);
    assert_eq!(datagram.payload(), b"cpu=42");
    assert_eq!(datagram.len(), 6);
}

#[test]
fn deterministic_best_effort_model_can_deliver_duplicate_or_drop() {
    let source = UdpEndpoint::new("sensor-a", 40_000);
    let destination = UdpEndpoint::new("colector", 8125);
    let datagram =
        UdpDatagram::new(source, destination, b"temperatura=31".to_vec()).expect("válido");

    assert_eq!(
        DeliveryOutcome::deterministic(datagram.clone(), 1),
        DeliveryOutcome::Delivered(datagram.clone())
    );
    assert_eq!(
        DeliveryOutcome::deterministic(datagram.clone(), 3),
        DeliveryOutcome::Duplicated(vec![datagram.clone(), datagram.clone()])
    );
    assert_eq!(
        DeliveryOutcome::deterministic(datagram, 5),
        DeliveryOutcome::Lost
    );
}

#[test]
fn payload_larger_than_udp_limit_is_rejected() {
    let source = UdpEndpoint::new("10.0.0.10", 50_000);
    let destination = UdpEndpoint::new("10.0.0.20", 8125);
    let oversized = vec![0; UdpDatagram::MAX_PAYLOAD_SIZE + 1];

    let error = UdpDatagram::new(source, destination, oversized).unwrap_err();

    assert_eq!(
        error,
        UdpError::PayloadTooLarge {
            size: UdpDatagram::MAX_PAYLOAD_SIZE + 1,
            max: UdpDatagram::MAX_PAYLOAD_SIZE,
        }
    );
}
