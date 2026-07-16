use rust_networking::http::HttpRequest;
use rust_networking::https::{HttpsRequest, SecureTransport};
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
        Certificate::new("api.jeresoft.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);

    let transporte = SecureTransport::connect(
        segura,
        cliente,
        cadena,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap();

    assert_eq!(transporte.tls().version(), TlsVersion::V1_3);
    println!("versión TLS segura: {:?}", transporte.tls().version());
}
