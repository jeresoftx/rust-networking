//! HTTP.
//!
//! Objetivo de aprendizaje: entender solicitud/respuesta, métodos,
//! encabezados, códigos de estado, cuerpo, persistencia de conexión y semántica
//! de caché.

use std::collections::BTreeMap;

/// Método HTTP modelado para solicitudes educativas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    fn parse(value: &str) -> Result<Self, HttpParseError> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "PATCH" => Ok(Self::Patch),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            other => Err(HttpParseError::InvalidMethod(other.to_string())),
        }
    }
}

/// Versión del protocolo anunciada en la línea inicial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http2,
}

impl HttpVersion {
    fn parse(value: &str) -> Result<Self, HttpParseError> {
        match value {
            "HTTP/1.0" => Ok(Self::Http10),
            "HTTP/1.1" => Ok(Self::Http11),
            "HTTP/2" | "HTTP/2.0" => Ok(Self::Http2),
            other => Err(HttpParseError::InvalidVersion(other.to_string())),
        }
    }
}

/// Errores del parser educativo de HTTP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpParseError {
    EmptyRequest,
    InvalidRequestLine(String),
    InvalidMethod(String),
    InvalidVersion(String),
    InvalidHeader(String),
}

/// Mapa de encabezados con búsqueda insensible a mayúsculas y minúsculas.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HeaderMap {
    entries: BTreeMap<String, String>,
}

impl HeaderMap {
    /// Crea un mapa vacío.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserta un encabezado. La clave se normaliza a minúsculas.
    pub fn insert(&mut self, name: impl AsRef<str>, value: impl Into<String>) {
        self.entries
            .insert(normalize_header_name(name.as_ref()), value.into());
    }

    /// Inserta un encabezado y regresa el mapa para composición fluida.
    pub fn with_header(mut self, name: impl AsRef<str>, value: impl Into<String>) -> Self {
        self.insert(name, value);
        self
    }

    /// Obtiene un encabezado por nombre sin importar mayúsculas.
    pub fn get(&self, name: impl AsRef<str>) -> Option<&str> {
        self.entries
            .get(&normalize_header_name(name.as_ref()))
            .map(String::as_str)
    }

    /// Inserta `Cache-Control`.
    pub fn with_cache_control(self, value: impl Into<String>) -> Self {
        self.with_header("Cache-Control", value)
    }

    /// Inserta `ETag`.
    pub fn with_etag(self, value: impl Into<String>) -> Self {
        self.with_header("ETag", value)
    }

    /// Lee `Cache-Control`.
    pub fn cache_control(&self) -> Option<&str> {
        self.get("Cache-Control")
    }

    /// Lee `ETag`.
    pub fn etag(&self) -> Option<&str> {
        self.get("ETag")
    }

    /// Número de encabezados.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Indica si el mapa está vacío.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Itera encabezados en orden estable.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
    }
}

/// Solicitud HTTP con línea inicial, encabezados y cuerpo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    version: HttpVersion,
    headers: HeaderMap,
    body: Vec<u8>,
}

impl HttpRequest {
    /// Parsea una solicitud HTTP/1.x limitada desde texto.
    ///
    /// Este parser es deliberadamente pequeño: enseña la anatomía del mensaje,
    /// pero no pretende reemplazar bibliotecas de producción.
    pub fn parse(raw: &str) -> Result<Self, HttpParseError> {
        if raw.trim().is_empty() {
            return Err(HttpParseError::EmptyRequest);
        }

        let (head, body) = split_head_body(raw);
        let mut lines = head.lines();
        let request_line = lines.next().ok_or(HttpParseError::EmptyRequest)?;
        let mut parts = request_line.split_whitespace();

        let method_text = parts
            .next()
            .ok_or_else(|| HttpParseError::InvalidRequestLine(request_line.to_string()))?;
        let path = parts
            .next()
            .ok_or_else(|| HttpParseError::InvalidRequestLine(request_line.to_string()))?;
        let version_text = parts
            .next()
            .ok_or_else(|| HttpParseError::InvalidRequestLine(request_line.to_string()))?;

        if parts.next().is_some() {
            return Err(HttpParseError::InvalidRequestLine(request_line.to_string()));
        }

        let mut headers = HeaderMap::new();
        for line in lines.filter(|line| !line.trim().is_empty()) {
            let (name, value) = line
                .split_once(':')
                .ok_or_else(|| HttpParseError::InvalidHeader(line.to_string()))?;
            if name.trim().is_empty() {
                return Err(HttpParseError::InvalidHeader(line.to_string()));
            }
            headers.insert(name.trim(), value.trim().to_string());
        }

        Ok(Self {
            method: HttpMethod::parse(method_text)?,
            path: path.to_string(),
            version: HttpVersion::parse(version_text)?,
            headers,
            body: body.as_bytes().to_vec(),
        })
    }

    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> HttpVersion {
        self.version
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

/// Código de estado frecuente en respuestas educativas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok,
    Created,
    NoContent,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    /// Código numérico enviado en la respuesta.
    pub fn code(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::NoContent => 204,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
        }
    }

    /// Frase legible asociada al código.
    pub fn reason_phrase(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Created => "Created",
            Self::NoContent => "No Content",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}

/// Respuesta HTTP educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Vec<u8>,
}

impl HttpResponse {
    /// Crea una respuesta sin encabezados ni cuerpo.
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body: Vec::new(),
        }
    }

    /// Agrega un encabezado.
    pub fn with_header(mut self, name: impl AsRef<str>, value: impl Into<String>) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Agrega cuerpo.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn reason_phrase(&self) -> &'static str {
        self.status.reason_phrase()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

fn normalize_header_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn split_head_body(raw: &str) -> (&str, &str) {
    if let Some((head, body)) = raw.split_once("\r\n\r\n") {
        (head, body)
    } else if let Some((head, body)) = raw.split_once("\n\n") {
        (head, body)
    } else {
        (raw, "")
    }
}
