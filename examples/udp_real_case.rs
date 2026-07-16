use rust_networking::udp::{DeliveryOutcome, UdpDatagram, UdpEndpoint};

fn main() {
    let sensor = UdpEndpoint::new("sensor-habitacion-7", 4_000);
    let colector = UdpEndpoint::new("telemetria", 8_125);

    let mut ultima_lectura = None;

    for secuencia in 1..=8 {
        let lectura = format!("habitacion=7 temperatura={}", 24 + secuencia);
        let datagrama =
            UdpDatagram::new(sensor.clone(), colector.clone(), lectura.into_bytes()).unwrap();

        match DeliveryOutcome::deterministic(datagrama, secuencia) {
            DeliveryOutcome::Delivered(datagrama) => {
                ultima_lectura = Some(String::from_utf8(datagrama.payload().to_vec()).unwrap());
            }
            DeliveryOutcome::Duplicated(copias) => {
                let primera = copias.first().expect("hay al menos una copia");
                ultima_lectura = Some(String::from_utf8(primera.payload().to_vec()).unwrap());
            }
            DeliveryOutcome::Lost => {
                println!("se perdió la lectura {secuencia}; se espera la siguiente");
            }
        }
    }

    println!("última lectura útil: {:?}", ultima_lectura);
}
