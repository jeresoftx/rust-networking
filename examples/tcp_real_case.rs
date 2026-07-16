use rust_networking::tcp::{SequenceNumber, TcpConnection, TcpState};

fn main() {
    let mut navegador = TcpConnection::new_client(SequenceNumber::new(1_000));
    let mut api = TcpConnection::new_server(SequenceNumber::new(9_000));

    let syn = navegador.open().unwrap();
    let syn_ack = api.receive(syn).unwrap().expect("api responde SYN+ACK");
    let ack = navegador
        .receive(syn_ack)
        .unwrap()
        .expect("navegador responde ACK");
    api.receive(ack).unwrap();

    assert_eq!(navegador.state(), TcpState::Established);
    assert_eq!(api.state(), TcpState::Established);

    let body = navegador
        .send_data(b"GET /reservas/GGG74R".to_vec())
        .unwrap();
    let acuse = api.receive(body).unwrap().expect("api acusa datos");

    println!(
        "solicitud reconstruida: {}",
        String::from_utf8(api.received_payload().to_vec()).unwrap()
    );
    println!("acuse enviado por la api: {:?}", acuse.ack_number());
}
