use std::hint::black_box;
use std::time::Instant;

use rust_networking::udp::{DeliveryOutcome, UdpDatagram, UdpEndpoint};

fn crear_datagramas_pequenos(iteraciones: usize) {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);

    for secuencia in 0..iteraciones {
        let carga = format!("temperatura={secuencia}").into_bytes();
        let datagrama = UdpDatagram::new(origen.clone(), destino.clone(), carga).unwrap();
        black_box(datagrama);
    }
}

fn rechazar_datagramas_demasiado_grandes(iteraciones: usize) {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);

    for _ in 0..iteraciones {
        let carga = vec![0; UdpDatagram::MAX_PAYLOAD_SIZE + 1];
        let error = UdpDatagram::new(origen.clone(), destino.clone(), carga).unwrap_err();
        black_box(error);
    }
}

fn despachar_deterministico(iteraciones: usize) {
    let origen = UdpEndpoint::new("sensor-a", 4_000);
    let destino = UdpEndpoint::new("colector", 8_125);
    let datagrama = UdpDatagram::new(origen, destino, vec![1, 2, 3, 4]).unwrap();

    for secuencia in 1..=iteraciones as u64 {
        let resultado = DeliveryOutcome::deterministic(datagrama.clone(), secuencia);
        black_box(resultado);
    }
}

fn main() {
    let iteraciones = 20_000usize;

    let start = Instant::now();
    crear_datagramas_pequenos(iteraciones);
    let create_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_datagramas_demasiado_grandes(iteraciones);
    let reject_elapsed = start.elapsed();

    let start = Instant::now();
    despachar_deterministico(iteraciones);
    let dispatch_elapsed = start.elapsed();

    println!("benchmark de udp (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("crear datagramas pequeños: {create_elapsed:?}");
    println!("rechazar datagramas demasiado grandes: {reject_elapsed:?}");
    println!("despacho determinista: {dispatch_elapsed:?}");
}
