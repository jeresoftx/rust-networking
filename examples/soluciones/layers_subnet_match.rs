use rust_networking::layers::{Ipv4Address, Ipv4Cidr};

fn main() {
    let subnet = Ipv4Cidr::new(Ipv4Address::new(10, 20, 0, 0), 16).unwrap();

    assert!(subnet.contains(Ipv4Address::new(10, 20, 30, 40)));
    assert!(!subnet.contains(Ipv4Address::new(10, 21, 0, 1)));
}
