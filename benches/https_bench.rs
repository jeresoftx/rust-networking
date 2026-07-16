use std::hint::black_box;
use std::time::Instant;

use rust_networking::http::HttpRequest;
use rust_networking::https::{HstsPolicy, HttpsPolicy, HttpsRequest, SecureTransport};
use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsVersion,
};

fn cadena(nombre: &str) -> CertificateChain {
    CertificateChain::new(vec![
        Certificate::new(nombre, "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ])
}

fn cliente(nombre: &str) -> TlsClientHello {
    TlsClientHello::new(
        nombre,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
}

fn solicitud() -> HttpsRequest {
    let http =
        HttpRequest::parse("GET /academy HTTP/1.1\r\nHost: api.jeresoft.test\r\n\r\n").unwrap();
    HttpsRequest::new("api.jeresoft.test", http)
}

fn conectar_valido(iteraciones: usize) {
    for _ in 0..iteraciones {
        let transporte = SecureTransport::connect(
            solicitud(),
            cliente("api.jeresoft.test"),
            cadena("api.jeresoft.test"),
            vec![TlsVersion::V1_3],
            vec![CipherSuite::TlsAes128GcmSha256],
        )
        .unwrap();
        black_box(transporte);
    }
}

fn rechazar_certificado(iteraciones: usize) {
    for _ in 0..iteraciones {
        let error = SecureTransport::connect(
            solicitud(),
            cliente("api.jeresoft.test"),
            cadena("otro.jeresoft.test"),
            vec![TlsVersion::V1_3],
            vec![CipherSuite::TlsAes128GcmSha256],
        )
        .unwrap_err();
        black_box(error);
    }
}

fn evaluar_hsts(iteraciones: usize) {
    let politica = HttpsPolicy::new(
        HstsPolicy::new()
            .include_host("api.jeresoft.test")
            .include_subdomains(),
    );

    for _ in 0..iteraciones {
        black_box(politica.should_force_https("api.jeresoft.test"));
        black_box(politica.should_force_https("v1.api.jeresoft.test"));
        black_box(politica.should_force_https("otro.jeresoft.test"));
    }
}

fn main() {
    let iteraciones = 20_000usize;

    let start = Instant::now();
    conectar_valido(iteraciones);
    let valido_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_certificado(iteraciones);
    let rechazo_elapsed = start.elapsed();

    let start = Instant::now();
    evaluar_hsts(iteraciones);
    let hsts_elapsed = start.elapsed();

    println!("benchmark de https (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("composición segura: {valido_elapsed:?}");
    println!("rechazo por certificado: {rechazo_elapsed:?}");
    println!("evaluación HSTS: {hsts_elapsed:?}");
}
