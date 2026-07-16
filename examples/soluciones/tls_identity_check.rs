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
        vec![CipherSuite::TlsAes128GcmSha256],
    );

    let servidor = TlsHandshake::negotiate(
        &cliente,
        cadena,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap();

    assert_eq!(servidor.version(), TlsVersion::V1_3);
}
