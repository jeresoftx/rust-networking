use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsError, TlsHandshake, TlsVersion,
};

#[test]
fn certificate_identity_must_match_requested_server_name() {
    let certificate = Certificate::new("api.jeresoft.test", "Jeresoft Academy CA");
    let chain = CertificateChain::new(vec![
        certificate,
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);
    let client = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_3, TlsVersion::V1_2],
        vec![CipherSuite::TlsAes128GcmSha256],
    );

    let server = TlsHandshake::negotiate(
        &client,
        chain,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap();

    assert_eq!(server.version(), TlsVersion::V1_3);
    assert_eq!(server.cipher_suite(), CipherSuite::TlsAes128GcmSha256);
    assert_eq!(server.certificate().subject(), "api.jeresoft.test");
}

#[test]
fn incomplete_certificate_chain_is_rejected() {
    let chain = CertificateChain::new(vec![Certificate::new(
        "api.jeresoft.test",
        "Jeresoft Academy CA",
    )]);
    let client = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    );

    let error = TlsHandshake::negotiate(
        &client,
        chain,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .unwrap_err();

    assert_eq!(error, TlsError::IncompleteCertificateChain);
}

#[test]
fn handshake_selects_highest_common_version_and_cipher_suite() {
    let chain = CertificateChain::new(vec![
        Certificate::new("api.jeresoft.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);
    let client = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_2, TlsVersion::V1_3],
        vec![
            CipherSuite::TlsChacha20Poly1305Sha256,
            CipherSuite::TlsAes256GcmSha384,
        ],
    );

    let server = TlsHandshake::negotiate(
        &client,
        chain,
        vec![TlsVersion::V1_3, TlsVersion::V1_2],
        vec![
            CipherSuite::TlsAes128GcmSha256,
            CipherSuite::TlsAes256GcmSha384,
        ],
    )
    .unwrap();

    assert_eq!(server.version(), TlsVersion::V1_3);
    assert_eq!(server.cipher_suite(), CipherSuite::TlsAes256GcmSha384);
}

#[test]
fn obsolete_cipher_suite_is_rejected() {
    let chain = CertificateChain::new(vec![
        Certificate::new("api.jeresoft.test", "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);
    let client = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_2],
        vec![CipherSuite::TlsRsaWithAes128CbcSha],
    );

    let error = TlsHandshake::negotiate(
        &client,
        chain,
        vec![TlsVersion::V1_2],
        vec![CipherSuite::TlsRsaWithAes128CbcSha],
    )
    .unwrap_err();

    assert_eq!(
        error,
        TlsError::ObsoleteCipherSuite(CipherSuite::TlsRsaWithAes128CbcSha)
    );
}
