use std::hint::black_box;
use std::time::Instant;

use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsHandshake, TlsVersion,
};

fn cadena_valida(nombre: &str) -> CertificateChain {
    CertificateChain::new(vec![
        Certificate::new(nombre, "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ])
}

fn cliente(nombre: &str, suites: Vec<CipherSuite>) -> TlsClientHello {
    TlsClientHello::new(nombre, vec![TlsVersion::V1_3, TlsVersion::V1_2], suites)
}

fn negociar_valido(iteraciones: usize) {
    for _ in 0..iteraciones {
        let cliente = cliente(
            "api.jeresoft.test",
            vec![
                CipherSuite::TlsChacha20Poly1305Sha256,
                CipherSuite::TlsAes256GcmSha384,
            ],
        );
        let resultado = TlsHandshake::negotiate(
            &cliente,
            cadena_valida("api.jeresoft.test"),
            vec![TlsVersion::V1_3, TlsVersion::V1_2],
            vec![
                CipherSuite::TlsAes128GcmSha256,
                CipherSuite::TlsAes256GcmSha384,
            ],
        )
        .unwrap();
        black_box(resultado);
    }
}

fn rechazar_identidad(iteraciones: usize) {
    for _ in 0..iteraciones {
        let cliente = cliente("api.jeresoft.test", vec![CipherSuite::TlsAes128GcmSha256]);
        let resultado = TlsHandshake::negotiate(
            &cliente,
            cadena_valida("otro.jeresoft.test"),
            vec![TlsVersion::V1_3],
            vec![CipherSuite::TlsAes128GcmSha256],
        )
        .unwrap_err();
        black_box(resultado);
    }
}

fn rechazar_suite_obsoleta(iteraciones: usize) {
    for _ in 0..iteraciones {
        let cliente = cliente(
            "api.jeresoft.test",
            vec![CipherSuite::TlsRsaWithAes128CbcSha],
        );
        let resultado = TlsHandshake::negotiate(
            &cliente,
            cadena_valida("api.jeresoft.test"),
            vec![TlsVersion::V1_2],
            vec![CipherSuite::TlsRsaWithAes128CbcSha],
        )
        .unwrap_err();
        black_box(resultado);
    }
}

fn main() {
    let iteraciones = 20_000usize;

    let start = Instant::now();
    negociar_valido(iteraciones);
    let valido_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_identidad(iteraciones);
    let identidad_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_suite_obsoleta(iteraciones);
    let suite_elapsed = start.elapsed();

    println!("benchmark de tls (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("negociación válida: {valido_elapsed:?}");
    println!("rechazo por identidad: {identidad_elapsed:?}");
    println!("rechazo por suite obsoleta: {suite_elapsed:?}");
}
