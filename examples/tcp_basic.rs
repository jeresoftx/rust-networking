use rust_networking::tcp::{SequenceNumber, TcpConnection, TcpState};

fn main() {
    let mut cliente = TcpConnection::new_client(SequenceNumber::new(100));

    let syn = cliente.open().unwrap();

    println!("segmento inicial: {:?}", syn);
    println!("es SYN: {}", syn.is_syn());
    println!("estado del cliente: {:?}", cliente.state());

    assert_eq!(cliente.state(), TcpState::SynSent);
}
