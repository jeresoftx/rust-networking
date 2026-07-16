use rust_networking::layers::{Ipv4Address, Ipv4Cidr};

fn main() {
    let subnet = Ipv4Cidr::new(Ipv4Address::new(192, 168, 10, 0), 24).unwrap();
    let address = Ipv4Address::new(192, 168, 10, 42);

    println!(
        "{address} pertenece a {subnet}: {}",
        subnet.contains(address)
    );
}
