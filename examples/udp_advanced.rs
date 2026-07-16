use rust_networking::udp::{DeliveryOutcome, UdpDatagram, UdpEndpoint};

fn main() {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);

    let mut entregados = 0usize;
    let mut duplicados = 0usize;
    let mut perdidos = 0usize;

    for secuencia in 1..=12 {
        let cuerpo = format!("sensor=a secuencia={secuencia}");
        let datagrama =
            UdpDatagram::new(origen.clone(), destino.clone(), cuerpo.into_bytes()).unwrap();

        match DeliveryOutcome::deterministic(datagrama, secuencia) {
            DeliveryOutcome::Delivered(_) => entregados += 1,
            DeliveryOutcome::Duplicated(copias) => duplicados += copias.len(),
            DeliveryOutcome::Lost => perdidos += 1,
        }
    }

    println!("entregados una vez: {entregados}");
    println!("copias duplicadas recibidas: {duplicados}");
    println!("perdidos: {perdidos}");
}
