use std::hint::black_box;
use std::time::Instant;

use rust_networking::tcp::{SequenceNumber, TcpConnection, TcpSegment};

fn deliver_in_order(iterations: usize, segments: usize) {
    for _ in 0..iterations {
        let mut receiver =
            TcpConnection::established(SequenceNumber::new(1), SequenceNumber::new(1));

        for index in 0..segments {
            let sequence = SequenceNumber::new(1 + index as u32);
            let respuesta = receiver
                .receive(TcpSegment::data(sequence, vec![b'x']))
                .unwrap();
            black_box(respuesta);
        }

        black_box(receiver.received_payload().len());
    }
}

fn deliver_after_reordering(iterations: usize, segments: usize) {
    for _ in 0..iterations {
        let mut receiver =
            TcpConnection::established(SequenceNumber::new(1), SequenceNumber::new(1));

        for index in (1..segments).rev() {
            let sequence = SequenceNumber::new(1 + index as u32);
            let respuesta = receiver
                .receive(TcpSegment::data(sequence, vec![b'x']))
                .unwrap();
            black_box(respuesta);
        }

        let respuesta = receiver
            .receive(TcpSegment::data(SequenceNumber::new(1), vec![b'x']))
            .unwrap();
        black_box(respuesta);
        black_box(receiver.received_payload().len());
    }
}

fn retransmit_pending(iterations: usize, pending: usize) {
    for _ in 0..iterations {
        let mut connection =
            TcpConnection::established(SequenceNumber::new(1), SequenceNumber::new(1));

        for index in 0..pending {
            let byte = b'a' + (index % 26) as u8;
            black_box(connection.send_data(vec![byte]).unwrap());
        }

        black_box(connection.retransmit_unacked().unwrap());
    }
}

fn main() {
    let iterations = 10_000usize;
    let segments = 32usize;

    let start = Instant::now();
    deliver_in_order(iterations, segments);
    let ordered_elapsed = start.elapsed();

    let start = Instant::now();
    deliver_after_reordering(iterations, segments);
    let reordered_elapsed = start.elapsed();

    let start = Instant::now();
    retransmit_pending(iterations, segments);
    let retransmission_elapsed = start.elapsed();

    println!("benchmark de tcp (manual, std::time::Instant)");
    println!("iteraciones: {iterations}");
    println!("segmentos por iteración: {segments}");
    println!("entrega en orden: {ordered_elapsed:?}");
    println!("entrega con reordenamiento: {reordered_elapsed:?}");
    println!("retransmisión de segmentos pendientes: {retransmission_elapsed:?}");
}
