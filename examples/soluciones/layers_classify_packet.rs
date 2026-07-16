use rust_networking::layers::{EncapsulatedFrame, NetworkLayer};

fn main() {
    let frame = EncapsulatedFrame::new("payload")
        .wrap(NetworkLayer::Application, "HTTP")
        .wrap(NetworkLayer::Transport, "TCP")
        .wrap(NetworkLayer::Internet, "IPv4");

    assert_eq!(frame.outermost_layer(), Some(NetworkLayer::Internet));
}
