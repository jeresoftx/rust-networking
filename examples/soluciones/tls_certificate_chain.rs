use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsError, TlsHandshake, TlsVersion,
};

fn main() {
    let cadena = CertificateChain::new(vec![Certificate::new(
        "api.jeresoft.test",
        "Jeresoft Academy CA",
    )]);
    let cliente = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    );

    let error = TlsHandshake::negotiate(
        &cliente,
        cadena,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap_err();

    assert_eq!(error, TlsError::IncompleteCertificateChain);
}
