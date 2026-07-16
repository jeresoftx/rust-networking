use rust_networking::layers::{EncapsulatedFrame, NetworkLayer};

fn main() {
    let frame = EncapsulatedFrame::new("GET /")
        .wrap(NetworkLayer::Application, "HTTP")
        .wrap(NetworkLayer::Transport, "TCP")
        .wrap(NetworkLayer::Internet, "IPv4");

    println!("capas: {}", frame.describe_path());
    println!("capa externa: {:?}", frame.outermost_layer());
}
