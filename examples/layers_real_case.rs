use rust_networking::layers::{
    EncapsulatedFrame, Ipv4Address, Ipv4Cidr, NetworkLayer, Route, RoutingTable,
};

fn main() {
    let mut frame = EncapsulatedFrame::new("GET https://api.example.com/v1/bookings")
        .wrap(NetworkLayer::Application, "HTTP")
        .wrap(NetworkLayer::Transport, "TCP")
        .wrap(NetworkLayer::Internet, "IPv4")
        .with_ttl(3);

    let table = RoutingTable::new(vec![
        Route::new(
            Ipv4Cidr::new(Ipv4Address::new(10, 0, 0, 0), 8).unwrap(),
            "vpn",
            None,
        ),
        Route::new(
            Ipv4Cidr::new(Ipv4Address::new(0, 0, 0, 0), 0).unwrap(),
            "internet",
            Some(Ipv4Address::new(192, 168, 1, 1)),
        ),
    ]);

    let api_address = Ipv4Address::new(203, 0, 113, 20);
    let decision = table
        .select_route(api_address)
        .expect("default route exists");

    println!("carga útil: {}", frame.payload());
    println!("encapsulación: {}", frame.describe_path());
    println!("ruta: {} vía {:?}", decision.interface, decision.next_hop);
    println!("ttl después de un salto: {:?}", frame.decrement_ttl());
}
