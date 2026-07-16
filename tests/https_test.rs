use rust_networking::http::{HttpMethod, HttpRequest};
use rust_networking::https::{HstsPolicy, HttpsError, HttpsPolicy, HttpsRequest, SecureTransport};
use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsError, TlsVersion,
};

fn cadena_valida(nombre: &str) -> CertificateChain {
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

#[test]
fn composes_http_request_over_valid_tls_session() {
    let http =
        HttpRequest::parse("GET /academy HTTP/1.1\r\nHost: api.jeresoft.test\r\n\r\n").unwrap();
    let secure_request = HttpsRequest::new("api.jeresoft.test", http);

    let transport = SecureTransport::connect(
        secure_request,
        cliente("api.jeresoft.test"),
        cadena_valida("api.jeresoft.test"),
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap();

    assert_eq!(transport.authority(), "api.jeresoft.test");
    assert_eq!(transport.request().method(), HttpMethod::Get);
    assert_eq!(transport.request().path(), "/academy");
    assert_eq!(transport.tls().version(), TlsVersion::V1_3);
}

#[test]
fn certificate_identity_failure_is_propagated() {
    let http =
        HttpRequest::parse("GET /academy HTTP/1.1\r\nHost: api.jeresoft.test\r\n\r\n").unwrap();
    let secure_request = HttpsRequest::new("api.jeresoft.test", http);

    let error = SecureTransport::connect(
        secure_request,
        cliente("api.jeresoft.test"),
        cadena_valida("otro.jeresoft.test"),
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap_err();

    assert_eq!(
        error,
        HttpsError::Tls(TlsError::ServerNameMismatch {
            expected: "api.jeresoft.test".to_string(),
            actual: "otro.jeresoft.test".to_string(),
        })
    );
}

#[test]
fn hsts_policy_forces_https_for_known_hosts() {
    let policy = HstsPolicy::new()
        .include_host("api.jeresoft.test")
        .include_subdomains();
    let https_policy = HttpsPolicy::new(policy);

    assert!(https_policy.should_force_https("api.jeresoft.test"));
    assert!(https_policy.should_force_https("v1.api.jeresoft.test"));
    assert!(!https_policy.should_force_https("otro.jeresoft.test"));
}
