use rust_networking::tcp::{
    SequenceNumber, TcpConnection, TcpError, TcpEvent, TcpSegment, TcpState,
};

#[test]
fn three_way_handshake_reaches_established_on_both_sides() {
    let mut client = TcpConnection::new_client(SequenceNumber::new(100));
    let mut server = TcpConnection::new_server(SequenceNumber::new(500));

    let syn = client.open().unwrap();
    assert!(syn.is_syn());
    assert_eq!(client.state(), TcpState::SynSent);

    let syn_ack = server.receive(syn).unwrap().expect("server replies");
    assert!(syn_ack.is_syn());
    assert!(syn_ack.is_ack());
    assert_eq!(syn_ack.ack_number(), Some(SequenceNumber::new(101)));
    assert_eq!(server.state(), TcpState::SynReceived);

    let final_ack = client.receive(syn_ack).unwrap().expect("client replies");
    assert!(final_ack.is_ack());
    assert!(!final_ack.is_syn());
    assert_eq!(final_ack.ack_number(), Some(SequenceNumber::new(501)));
    assert_eq!(client.state(), TcpState::Established);

    assert_eq!(server.receive(final_ack).unwrap(), None);
    assert_eq!(server.state(), TcpState::Established);
    assert!(server.events().contains(&TcpEvent::ConnectionEstablished));
}

#[test]
fn unexpected_segment_is_rejected_for_current_state() {
    let mut connection = TcpConnection::new_client(SequenceNumber::new(10));

    let error = connection
        .receive(TcpSegment::ack(
            SequenceNumber::new(1),
            SequenceNumber::new(1),
        ))
        .unwrap_err();

    assert_eq!(
        error,
        TcpError::UnexpectedSegment {
            state: TcpState::Closed,
            flags: "ACK".to_string(),
        }
    );
}

#[test]
fn receiver_buffers_out_of_order_segments_until_gap_is_filled() {
    let mut receiver = TcpConnection::established(SequenceNumber::new(1), SequenceNumber::new(1));

    let late = TcpSegment::data(SequenceNumber::new(6), b"world".to_vec());
    assert!(receiver.receive(late).unwrap().is_some());
    assert_eq!(receiver.pending_segments(), 1);
    assert!(receiver.received_payload().is_empty());

    let first = TcpSegment::data(SequenceNumber::new(1), b"hello".to_vec());
    assert!(receiver.receive(first).unwrap().is_some());

    assert_eq!(receiver.pending_segments(), 0);
    assert_eq!(receiver.received_payload(), b"helloworld");
    assert!(receiver.events().contains(&TcpEvent::DataDelivered));
}

#[test]
fn established_connection_sends_data_and_tracks_unacknowledged_segments() {
    let mut sender = TcpConnection::established(SequenceNumber::new(20), SequenceNumber::new(80));

    let first = sender.send_data(b"hello".to_vec()).unwrap();
    let second = sender.send_data(b"world".to_vec()).unwrap();

    assert_eq!(first.sequence_number(), SequenceNumber::new(20));
    assert_eq!(second.sequence_number(), SequenceNumber::new(25));
    assert_eq!(sender.unacked_segments(), &[first, second]);
}

#[test]
fn unacknowledged_segment_can_be_retransmitted() {
    let mut client = TcpConnection::new_client(SequenceNumber::new(10));
    let syn = client.open().unwrap();

    assert_eq!(client.unacked_segments(), std::slice::from_ref(&syn));
    assert_eq!(client.retransmit_unacked().unwrap(), vec![syn]);
    assert!(client.events().contains(&TcpEvent::RetransmissionScheduled));
}

#[test]
fn fin_closes_an_established_connection_after_acknowledgement() {
    let mut connection =
        TcpConnection::established(SequenceNumber::new(20), SequenceNumber::new(80));

    let fin = connection.close().unwrap();

    assert!(fin.is_fin());
    assert_eq!(fin.sequence_number(), SequenceNumber::new(20));
    assert_eq!(connection.state(), TcpState::FinWait1);

    let ack = TcpSegment::ack(SequenceNumber::new(80), SequenceNumber::new(21));
    assert_eq!(connection.receive(ack).unwrap(), None);

    assert_eq!(connection.state(), TcpState::Closed);
    assert!(connection.events().contains(&TcpEvent::ConnectionClosed));
}
