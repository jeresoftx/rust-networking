use std::hint::black_box;
use std::time::Instant;

use rust_networking::layers::{Ipv4Address, Ipv4Cidr, Route, RoutingTable};

fn build_table(size: usize) -> RoutingTable {
    let mut routes = vec![Route::new(
        Ipv4Cidr::new(Ipv4Address::new(0, 0, 0, 0), 0).unwrap(),
        "default",
        None,
    )];

    for index in 0..size {
        let second = (index % 256) as u8;
        routes.push(Route::new(
            Ipv4Cidr::new(Ipv4Address::new(10, second, 0, 0), 16).unwrap(),
            format!("net-{second}"),
            None,
        ));
    }

    RoutingTable::new(routes)
}

fn main() {
    let iterations = 100_000usize;
    let destination = Ipv4Address::new(10, 42, 12, 1);
    let subnet = Ipv4Cidr::new(Ipv4Address::new(10, 42, 0, 0), 16).unwrap();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(subnet.contains(destination));
    }
    let cidr_elapsed = start.elapsed();

    let small_table = build_table(8);
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(small_table.select_route(destination));
    }
    let small_elapsed = start.elapsed();

    let medium_table = build_table(128);
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(medium_table.select_route(destination));
    }
    let medium_elapsed = start.elapsed();

    println!("benchmark de capas (manual, std::time::Instant)");
    println!("iterations: {iterations}");
    println!("pertenencia CIDR: {cidr_elapsed:?}");
    println!("búsqueda en tabla pequeña: {small_elapsed:?}");
    println!("búsqueda en tabla mediana: {medium_elapsed:?}");
}
