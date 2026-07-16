use rust_networking::http::HttpRequest;
use rust_networking::https::{HttpsError, HttpsRequest, SecureTransport};
use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsVersion,
};

fn main() {
    let solicitud =
        HttpRequest::parse("GET /academy HTTP/1.1\r\nHost: api.jeresoft.test\r\n\r\n").unwrap();
    let segura = HttpsRequest::new("api.jeresoft.test", solicitud);
    let cliente = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    );
    let cadena = CertificateChain::new(vec![
        Certificate::new("otro.jeresoft.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);

    let error = SecureTransport::connect(
        segura,
        cliente,
        cadena,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap_err();

    assert!(matches!(error, HttpsError::Tls(_)));
    println!("certificado rechazado: {error:?}");
}
