use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsHandshake, TlsVersion,
};

fn main() {
    let cadena_api = CertificateChain::new(vec![
        Certificate::new("api.reservas.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);
    let cliente = TlsClientHello::new(
        "api.reservas.test",
        vec![TlsVersion::V1_3, TlsVersion::V1_2],
        vec![
            CipherSuite::TlsAes128GcmSha256,
            CipherSuite::TlsChacha20Poly1305Sha256,
        ],
    );

    let negociacion = TlsHandshake::negotiate(
        &cliente,
        cadena_api,
        vec![TlsVersion::V1_3],
        vec![
            CipherSuite::TlsAes128GcmSha256,
            CipherSuite::TlsAes256GcmSha384,
        ],
    )
    .unwrap();

    println!("API autenticada: {}", negociacion.certificate().subject());
    println!("versión acordada: {:?}", negociacion.version());
    println!("suite acordada: {:?}", negociacion.cipher_suite());
}
