use rust_networking::tcp::{SequenceNumber, TcpConnection, TcpState};

fn main() {
    let mut cliente = TcpConnection::new_client(SequenceNumber::new(100));
    let mut servidor = TcpConnection::new_server(SequenceNumber::new(500));

    let syn = cliente.open().unwrap();
    let syn_ack = servidor.receive(syn).unwrap().expect("servidor responde");
    let ack = cliente.receive(syn_ack).unwrap().expect("cliente responde");
    servidor.receive(ack).unwrap();

    println!("cliente: {:?}", cliente.state());
    println!("servidor: {:?}", servidor.state());

    assert_eq!(cliente.state(), TcpState::Established);
    assert_eq!(servidor.state(), TcpState::Established);
}
