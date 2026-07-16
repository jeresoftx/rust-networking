//! HTTPS.
//!
//! Objetivo de aprendizaje: entender HTTP sobre TLS, identidad del servidor,
//! autoridad, HSTS y errores de configuración.

use crate::http::HttpRequest;
use crate::tls::{
    CertificateChain, CipherSuite, TlsClientHello, TlsError, TlsHandshake, TlsServerHello,
    TlsVersion,
};

/// Solicitud HTTPS educativa: una solicitud HTTP asociada a una autoridad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpsRequest {
    authority: String,
    request: HttpRequest,
}

impl HttpsRequest {
    /// Crea una solicitud segura para una autoridad esperada.
    pub fn new(authority: impl Into<String>, request: HttpRequest) -> Self {
        Self {
            authority: authority.into(),
            request,
        }
    }

    /// Autoridad que debe coincidir con la identidad TLS.
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Solicitud HTTP transportada.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }
}

/// Política HSTS educativa.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HstsPolicy {
    hosts: Vec<String>,
    include_subdomains: bool,
}

impl HstsPolicy {
    /// Crea una política vacía.
    pub fn new() -> Self {
        Self::default()
    }

    /// Agrega un host que debe usar HTTPS.
    pub fn include_host(mut self, host: impl Into<String>) -> Self {
        self.hosts.push(host.into());
        self
    }

    /// Fuerza subdominios de los hosts registrados.
    pub fn include_subdomains(mut self) -> Self {
        self.include_subdomains = true;
        self
    }

    /// Decide si un host debe usar HTTPS.
    pub fn should_force_https(&self, host: &str) -> bool {
        self.hosts.iter().any(|registered| {
            host == registered || (self.include_subdomains && is_subdomain_of(host, registered))
        })
    }

    /// Hosts registrados.
    pub fn hosts(&self) -> &[String] {
        &self.hosts
    }

    /// Indica si la regla cubre subdominios.
    pub fn covers_subdomains(&self) -> bool {
        self.include_subdomains
    }
}

/// Política HTTPS que agrupa reglas de transporte seguro.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpsPolicy {
    hsts: HstsPolicy,
}

impl HttpsPolicy {
    /// Crea una política HTTPS con HSTS educativo.
    pub fn new(hsts: HstsPolicy) -> Self {
        Self { hsts }
    }

    /// Decide si una autoridad debe forzarse a HTTPS.
    pub fn should_force_https(&self, host: &str) -> bool {
        self.hsts.should_force_https(host)
    }

    /// Devuelve la política HSTS.
    pub fn hsts(&self) -> &HstsPolicy {
        &self.hsts
    }
}

/// Transporte seguro resultante de componer HTTP sobre TLS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecureTransport {
    request: HttpsRequest,
    tls: TlsServerHello,
}

impl SecureTransport {
    /// Negocia TLS y conserva la solicitud HTTP si la autoridad queda protegida.
    pub fn connect(
        request: HttpsRequest,
        client: TlsClientHello,
        chain: CertificateChain,
        server_versions: Vec<TlsVersion>,
        server_cipher_suites: Vec<CipherSuite>,
    ) -> Result<Self, HttpsError> {
        if request.authority() != client.server_name() {
            return Err(HttpsError::AuthorityMismatch {
                expected: request.authority().to_string(),
                actual: client.server_name().to_string(),
            });
        }

        let tls = TlsHandshake::negotiate(&client, chain, server_versions, server_cipher_suites)
            .map_err(HttpsError::Tls)?;

        Ok(Self { request, tls })
    }

    /// Autoridad protegida.
    pub fn authority(&self) -> &str {
        self.request.authority()
    }

    /// Solicitud HTTP enviada sobre TLS.
    pub fn request(&self) -> &HttpRequest {
        self.request.request()
    }

    /// Resultado de la negociación TLS.
    pub fn tls(&self) -> &TlsServerHello {
        &self.tls
    }
}

/// Error educativo de HTTPS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpsError {
    /// La autoridad HTTP esperada no coincide con el nombre solicitado en TLS.
    AuthorityMismatch { expected: String, actual: String },
    /// Error propagado desde la negociación TLS.
    Tls(TlsError),
}

fn is_subdomain_of(host: &str, registered: &str) -> bool {
    host.len() > registered.len()
        && host.ends_with(registered)
        && host
            .as_bytes()
            .get(host.len() - registered.len() - 1)
            .is_some_and(|separator| *separator == b'.')
}
