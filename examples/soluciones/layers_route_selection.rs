use rust_networking::layers::{Ipv4Address, Ipv4Cidr, Route, RoutingTable};

fn main() {
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
            None,
        ),
    ]);

    let decision = table
        .select_route(Ipv4Address::new(10, 20, 30, 40))
        .expect("route exists");

    assert_eq!(decision.interface, "branch-a");
}
