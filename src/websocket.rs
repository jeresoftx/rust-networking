//! WebSocket.
//!
//! Objetivo de aprendizaje: entender actualización desde HTTP, tramas,
//! mensajes, ping/pong y cierre de conexión.

use crate::http::{HttpMethod, HttpRequest};

/// Código de operación de una trama WebSocket educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Text,
    Binary,
    Ping,
    Pong,
    Close,
}

/// Código de cierre frecuente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseCode {
    NormalClosure,
    GoingAway,
    ProtocolError,
}

impl CloseCode {
    /// Código numérico usado por WebSocket.
    pub fn code(self) -> u16 {
        match self {
            Self::NormalClosure => 1000,
            Self::GoingAway => 1001,
            Self::ProtocolError => 1002,
        }
    }
}

/// Estado educativo de una conexión WebSocket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// Solicitud de actualización desde HTTP hacia WebSocket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketUpgrade {
    request: HttpRequest,
}

impl WebSocketUpgrade {
    /// Crea una actualización pendiente desde una solicitud HTTP.
    pub fn new(request: HttpRequest) -> Self {
        Self { request }
    }

    /// Valida encabezados mínimos y abre la conexión educativa.
    pub fn accept(self) -> Result<WebSocketConnection, WebSocketError> {
        if self.request.method() != HttpMethod::Get {
            return Err(WebSocketError::InvalidMethod);
        }

        require_header_contains(&self.request, "Connection", "upgrade")?;
        require_header_equals(&self.request, "Upgrade", "websocket")?;

        if self.request.headers().get("Sec-WebSocket-Key").is_none() {
            return Err(WebSocketError::MissingWebSocketKey);
        }

        Ok(WebSocketConnection {
            path: self.request.path().to_string(),
            state: WebSocketState::Open,
            received: Vec::new(),
        })
    }
}

/// Conexión WebSocket educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketConnection {
    path: String,
    state: WebSocketState,
    received: Vec<WebSocketFrame>,
}

impl WebSocketConnection {
    /// Ruta HTTP que fue actualizada.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Estado actual de la conexión.
    pub fn state(&self) -> WebSocketState {
        self.state
    }

    /// Tramas de datos recibidas.
    pub fn received(&self) -> &[WebSocketFrame] {
        &self.received
    }

    /// Aplica una trama al estado educativo.
    pub fn apply_frame(
        &mut self,
        frame: WebSocketFrame,
    ) -> Result<Option<WebSocketFrame>, WebSocketError> {
        if self.state == WebSocketState::Closed {
            return Err(WebSocketError::ConnectionClosed);
        }

        match frame.opcode {
            Opcode::Close => {
                self.state = WebSocketState::Closed;
                Ok(None)
            }
            Opcode::Ping => Ok(frame.respond_to_ping()),
            Opcode::Text | Opcode::Binary => {
                self.received.push(frame);
                Ok(None)
            }
            Opcode::Pong => Ok(None),
        }
    }
}

/// Trama WebSocket educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketFrame {
    opcode: Opcode,
    payload: Vec<u8>,
    close_code: Option<CloseCode>,
}

impl WebSocketFrame {
    /// Crea una trama de texto.
    pub fn text(message: impl AsRef<str>) -> Self {
        Self {
            opcode: Opcode::Text,
            payload: message.as_ref().as_bytes().to_vec(),
            close_code: None,
        }
    }

    /// Crea una trama binaria.
    pub fn binary(payload: Vec<u8>) -> Self {
        Self {
            opcode: Opcode::Binary,
            payload,
            close_code: None,
        }
    }

    /// Crea una trama ping.
    pub fn ping(payload: Vec<u8>) -> Self {
        Self {
            opcode: Opcode::Ping,
            payload,
            close_code: None,
        }
    }

    /// Crea una trama pong.
    pub fn pong(payload: Vec<u8>) -> Self {
        Self {
            opcode: Opcode::Pong,
            payload,
            close_code: None,
        }
    }

    /// Crea una trama de cierre.
    pub fn close(code: CloseCode) -> Self {
        Self {
            opcode: Opcode::Close,
            payload: code.code().to_be_bytes().to_vec(),
            close_code: Some(code),
        }
    }

    /// Código de operación.
    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    /// Carga útil de la trama.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Código de cierre, si aplica.
    pub fn close_code(&self) -> Option<CloseCode> {
        self.close_code
    }

    /// Indica si la trama es de control.
    pub fn is_control(&self) -> bool {
        matches!(self.opcode, Opcode::Ping | Opcode::Pong | Opcode::Close)
    }

    /// Responde a ping con pong conservando la carga útil.
    pub fn respond_to_ping(&self) -> Option<Self> {
        (self.opcode == Opcode::Ping).then(|| Self::pong(self.payload.clone()))
    }
}

/// Error educativo de WebSocket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebSocketError {
    InvalidMethod,
    InvalidUpgradeHeader { name: String },
    MissingWebSocketKey,
    ConnectionClosed,
}

fn require_header_equals(
    request: &HttpRequest,
    name: &str,
    expected: &str,
) -> Result<(), WebSocketError> {
    let value =
        request
            .headers()
            .get(name)
            .ok_or_else(|| WebSocketError::InvalidUpgradeHeader {
                name: name.to_string(),
            })?;

    if value.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(WebSocketError::InvalidUpgradeHeader {
            name: name.to_string(),
        })
    }
}

fn require_header_contains(
    request: &HttpRequest,
    name: &str,
    expected_token: &str,
) -> Result<(), WebSocketError> {
    let value =
        request
            .headers()
            .get(name)
            .ok_or_else(|| WebSocketError::InvalidUpgradeHeader {
                name: name.to_string(),
            })?;

    if value
        .split(',')
        .any(|token| token.trim().eq_ignore_ascii_case(expected_token))
    {
        Ok(())
    } else {
        Err(WebSocketError::InvalidUpgradeHeader {
            name: name.to_string(),
        })
    }
}
