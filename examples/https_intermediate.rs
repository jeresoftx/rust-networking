use rust_networking::http::HttpRequest;
use rust_networking::https::{HttpsError, HttpsRequest, SecureTransport};
use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsVersion,
};

fn main() {
    let solicitud_http =
        HttpRequest::parse("GET /academy HTTP/1.1\r\nHost: api.jeresoft.test\r\n\r\n").unwrap();
    let solicitud_https = HttpsRequest::new("api.jeresoft.test", solicitud_http);
    let cliente = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    );
    let cadena_erronea = CertificateChain::new(vec![
        Certificate::new("otro.jeresoft.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);

    let error = SecureTransport::connect(
        solicitud_https,
        cliente,
        cadena_erronea,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap_err();

    if let HttpsError::Tls(error_tls) = error {
        println!("TLS rechazó la conexión: {error_tls:?}");
    }
}
