use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsError, TlsHandshake, TlsVersion,
};

fn main() {
    let cadena = CertificateChain::new(vec![
        Certificate::new("api.jeresoft.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);
    let cliente = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_2],
        vec![CipherSuite::TlsRsaWithAes128CbcSha],
    );

    let error = TlsHandshake::negotiate(
        &cliente,
        cadena,
        vec![TlsVersion::V1_2],
        vec![CipherSuite::TlsRsaWithAes128CbcSha],
    )
    .unwrap_err();

    println!("rechazo esperado: {error:?}");
    assert_eq!(
        error,
        TlsError::ObsoleteCipherSuite(CipherSuite::TlsRsaWithAes128CbcSha)
    );
}
