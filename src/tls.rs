//! TLS.
//!
//! Objetivo de aprendizaje: entender confidencialidad, integridad,
//! autenticación, negociación y certificados sin implementar criptografía de
//! producción.

/// Certificado educativo.
///
/// No contiene claves, firmas ni criptografía real. Solo modela identidad e
/// emisor para razonar sobre verificación estructural.
///
/// # Examples
///
/// ```
/// use rust_networking::tls::Certificate;
///
/// let certificate = Certificate::new("api.jeresoft.test", "Jeresoft Academy CA");
/// assert_eq!(certificate.subject(), "api.jeresoft.test");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Certificate {
    subject: String,
    issuer: String,
}

impl Certificate {
    /// Crea un certificado educativo.
    ///
    /// Complejidad: O(s + i), donde `s` es el sujeto e `i` el emisor.
    pub fn new(subject: impl Into<String>, issuer: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            issuer: issuer.into(),
        }
    }

    /// Devuelve el sujeto del certificado.
    ///
    /// Complejidad: O(1).
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Devuelve el emisor del certificado.
    ///
    /// Complejidad: O(1).
    pub fn issuer(&self) -> &str {
        &self.issuer
    }
}

/// Cadena de certificados educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateChain {
    certificates: Vec<Certificate>,
}

impl CertificateChain {
    /// Crea una cadena con el certificado de hoja primero.
    ///
    /// Complejidad: O(1) más la propiedad del vector recibido.
    pub fn new(certificates: Vec<Certificate>) -> Self {
        Self { certificates }
    }

    /// Devuelve el certificado de hoja.
    ///
    /// Complejidad: O(1).
    pub fn leaf(&self) -> Option<&Certificate> {
        self.certificates.first()
    }

    /// Devuelve todos los certificados.
    ///
    /// Complejidad: O(1).
    pub fn certificates(&self) -> &[Certificate] {
        &self.certificates
    }

    fn validate_structure(&self) -> Result<(), TlsError> {
        if self.certificates.len() < 2 {
            return Err(TlsError::IncompleteCertificateChain);
        }

        for pair in self.certificates.windows(2) {
            let child = &pair[0];
            let issuer = &pair[1];
            if child.issuer() != issuer.subject() {
                return Err(TlsError::BrokenCertificateChain {
                    subject: child.subject().to_string(),
                    expected_issuer: child.issuer().to_string(),
                    actual_issuer: issuer.subject().to_string(),
                });
            }
        }

        Ok(())
    }
}

/// Versión TLS educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TlsVersion {
    /// TLS 1.2.
    V1_2,
    /// TLS 1.3.
    V1_3,
}

/// Suite criptográfica educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CipherSuite {
    /// TLS_AES_128_GCM_SHA256.
    TlsAes128GcmSha256,
    /// TLS_AES_256_GCM_SHA384.
    TlsAes256GcmSha384,
    /// TLS_CHACHA20_POLY1305_SHA256.
    TlsChacha20Poly1305Sha256,
    /// TLS_RSA_WITH_AES_128_CBC_SHA. Se incluye para enseñar rechazo.
    TlsRsaWithAes128CbcSha,
}

impl CipherSuite {
    /// Indica si la suite está permitida por la política educativa.
    ///
    /// Complejidad: O(1).
    pub fn is_allowed(self) -> bool {
        !matches!(self, Self::TlsRsaWithAes128CbcSha)
    }
}

/// Mensaje ClientHello educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsClientHello {
    server_name: String,
    versions: Vec<TlsVersion>,
    cipher_suites: Vec<CipherSuite>,
}

impl TlsClientHello {
    /// Crea un ClientHello con nombre esperado, versiones y suites.
    ///
    /// Complejidad: O(n) más la propiedad de los vectores recibidos.
    pub fn new(
        server_name: impl Into<String>,
        versions: Vec<TlsVersion>,
        cipher_suites: Vec<CipherSuite>,
    ) -> Self {
        Self {
            server_name: server_name.into(),
            versions,
            cipher_suites,
        }
    }

    /// Devuelve el nombre de servidor esperado.
    ///
    /// Complejidad: O(1).
    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    /// Devuelve versiones soportadas por el cliente.
    ///
    /// Complejidad: O(1).
    pub fn versions(&self) -> &[TlsVersion] {
        &self.versions
    }

    /// Devuelve suites soportadas por el cliente.
    ///
    /// Complejidad: O(1).
    pub fn cipher_suites(&self) -> &[CipherSuite] {
        &self.cipher_suites
    }
}

/// Mensaje ServerHello educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsServerHello {
    version: TlsVersion,
    cipher_suite: CipherSuite,
    certificate: Certificate,
}

impl TlsServerHello {
    /// Devuelve la versión negociada.
    ///
    /// Complejidad: O(1).
    pub fn version(&self) -> TlsVersion {
        self.version
    }

    /// Devuelve la suite negociada.
    ///
    /// Complejidad: O(1).
    pub fn cipher_suite(&self) -> CipherSuite {
        self.cipher_suite
    }

    /// Devuelve el certificado de hoja.
    ///
    /// Complejidad: O(1).
    pub fn certificate(&self) -> &Certificate {
        &self.certificate
    }
}

/// Negociador TLS educativo.
pub struct TlsHandshake;

impl TlsHandshake {
    /// Negocia versión, suite e identidad sin criptografía real.
    ///
    /// Complejidad: O(v log v + c * s), donde `v` son versiones, `c` suites del
    /// cliente y `s` suites del servidor.
    pub fn negotiate(
        client: &TlsClientHello,
        chain: CertificateChain,
        server_versions: Vec<TlsVersion>,
        server_cipher_suites: Vec<CipherSuite>,
    ) -> Result<TlsServerHello, TlsError> {
        chain.validate_structure()?;
        let certificate = chain
            .leaf()
            .cloned()
            .ok_or(TlsError::IncompleteCertificateChain)?;

        if certificate.subject() != client.server_name() {
            return Err(TlsError::ServerNameMismatch {
                expected: client.server_name().to_string(),
                actual: certificate.subject().to_string(),
            });
        }

        let version = negotiate_version(client.versions(), &server_versions)?;
        let cipher_suite = negotiate_cipher_suite(client.cipher_suites(), &server_cipher_suites)?;

        Ok(TlsServerHello {
            version,
            cipher_suite,
            certificate,
        })
    }
}

fn negotiate_version(
    client_versions: &[TlsVersion],
    server_versions: &[TlsVersion],
) -> Result<TlsVersion, TlsError> {
    client_versions
        .iter()
        .filter(|version| server_versions.contains(version))
        .copied()
        .max()
        .ok_or(TlsError::NoSharedVersion)
}

fn negotiate_cipher_suite(
    client_cipher_suites: &[CipherSuite],
    server_cipher_suites: &[CipherSuite],
) -> Result<CipherSuite, TlsError> {
    for suite in client_cipher_suites {
        if server_cipher_suites.contains(suite) {
            if suite.is_allowed() {
                return Ok(*suite);
            }

            return Err(TlsError::ObsoleteCipherSuite(*suite));
        }
    }

    Err(TlsError::NoSharedCipherSuite)
}

/// Error educativo de negociación TLS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlsError {
    /// El certificado de hoja no coincide con el nombre solicitado.
    ServerNameMismatch { expected: String, actual: String },
    /// La cadena no contiene al menos hoja e intermediario.
    IncompleteCertificateChain,
    /// La cadena no enlaza sujeto y emisor.
    BrokenCertificateChain {
        subject: String,
        expected_issuer: String,
        actual_issuer: String,
    },
    /// Cliente y servidor no comparten versión.
    NoSharedVersion,
    /// Cliente y servidor no comparten suite permitida.
    NoSharedCipherSuite,
    /// La suite compartida está prohibida por la política educativa.
    ObsoleteCipherSuite(CipherSuite),
}
