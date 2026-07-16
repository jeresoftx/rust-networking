use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsHandshake, TlsVersion,
};

fn main() {
    let cadena = CertificateChain::new(vec![
        Certificate::new("api.jeresoft.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);
    let cliente = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_3],
        vec![
            CipherSuite::TlsChacha20Poly1305Sha256,
            CipherSuite::TlsAes256GcmSha384,
        ],
    );

    let servidor = TlsHandshake::negotiate(
        &cliente,
        cadena,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes256GcmSha384],
    )
    .unwrap();

    assert_eq!(servidor.cipher_suite(), CipherSuite::TlsAes256GcmSha384);
}
