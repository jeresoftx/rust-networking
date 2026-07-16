use rust_networking::layers::{
    EncapsulatedFrame, Ipv4Address, Ipv4Cidr, NetworkLayer, Route, RouteDecision, RoutingTable,
    TtlError,
};

#[test]
fn encapsulated_frame_records_layers_from_inner_to_outer() {
    let frame = EncapsulatedFrame::new("GET /")
        .wrap(NetworkLayer::Application, "HTTP request")
        .wrap(NetworkLayer::Transport, "TCP segment")
        .wrap(NetworkLayer::Internet, "IPv4 packet")
        .wrap(NetworkLayer::Link, "Ethernet frame");

    assert_eq!(
        frame.layers(),
        &[
            NetworkLayer::Application,
            NetworkLayer::Transport,
            NetworkLayer::Internet,
            NetworkLayer::Link,
        ]
    );
    assert_eq!(frame.outermost_layer(), Some(NetworkLayer::Link));
    assert_eq!(
        frame.describe_path(),
        "Application -> Transport -> Internet -> Link"
    );
}

#[test]
fn ipv4_cidr_checks_membership_with_prefix_mask() {
    let cidr = Ipv4Cidr::new(Ipv4Address::new(192, 168, 10, 0), 24).unwrap();

    assert!(cidr.contains(Ipv4Address::new(192, 168, 10, 42)));
    assert!(cidr.contains(Ipv4Address::new(192, 168, 10, 255)));
    assert!(!cidr.contains(Ipv4Address::new(192, 168, 11, 1)));
}

#[test]
fn routing_table_selects_the_most_specific_matching_route() {
    let table = RoutingTable::new(vec![
        Route::new(
            Ipv4Cidr::new(Ipv4Address::new(0, 0, 0, 0), 0).unwrap(),
            "wan",
            None,
        ),
        Route::new(
            Ipv4Cidr::new(Ipv4Address::new(10, 0, 0, 0), 8).unwrap(),
            "private",
            None,
        ),
        Route::new(
            Ipv4Cidr::new(Ipv4Address::new(10, 20, 0, 0), 16).unwrap(),
            "branch-a",
            Some(Ipv4Address::new(10, 20, 0, 1)),
        ),
    ]);

    let decision = table.select_route(Ipv4Address::new(10, 20, 30, 40));

    assert_eq!(
        decision,
        Some(RouteDecision {
            destination: Ipv4Address::new(10, 20, 30, 40),
            interface: "branch-a".to_string(),
            next_hop: Some(Ipv4Address::new(10, 20, 0, 1)),
            matched_prefix: Ipv4Cidr::new(Ipv4Address::new(10, 20, 0, 0), 16).unwrap(),
        })
    );
}

#[test]
fn packet_ttl_expires_before_forwarding_when_it_reaches_zero() {
    let mut frame = EncapsulatedFrame::new("payload").with_ttl(1);

    let error = frame.decrement_ttl().unwrap_err();

    assert_eq!(error, TtlError::Expired);
    assert_eq!(frame.ttl(), 0);
}
