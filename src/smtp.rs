//! SMTP.
//!
//! Objetivo de aprendizaje: entender sesiones de correo, sobre del mensaje,
//! encabezados, registros MX y límites del correo electrónico.

use std::collections::BTreeMap;

/// Comando SMTP educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmtpCommand {
    Helo(String),
    MailFrom(String),
    RcptTo(String),
    Data,
    MessageData(String),
    Quit,
}

impl SmtpCommand {
    fn name(&self) -> &'static str {
        match self {
            Self::Helo(_) => "HELO",
            Self::MailFrom(_) => "MAIL FROM",
            Self::RcptTo(_) => "RCPT TO",
            Self::Data => "DATA",
            Self::MessageData(_) => "message data",
            Self::Quit => "QUIT",
        }
    }
}

/// Respuesta SMTP educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmtpReply {
    code: u16,
    message: String,
}

impl SmtpReply {
    /// Crea una respuesta con código y texto.
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Código numérico SMTP.
    pub fn code(&self) -> u16 {
        self.code
    }

    /// Texto legible de la respuesta.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Sobre SMTP: remitente de rebote y destinatarios de entrega.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailEnvelope {
    sender: String,
    recipients: Vec<String>,
}

impl MailEnvelope {
    /// Crea un sobre con remitente.
    pub fn new(sender: impl Into<String>) -> Self {
        Self {
            sender: sender.into(),
            recipients: Vec::new(),
        }
    }

    /// Agrega un destinatario y devuelve el sobre para composición.
    pub fn add_recipient(mut self, recipient: impl Into<String>) -> Result<Self, SmtpError> {
        self.push_recipient(recipient)?;
        Ok(self)
    }

    /// Remitente del sobre.
    pub fn sender(&self) -> &str {
        &self.sender
    }

    /// Destinatarios del sobre.
    pub fn recipients(&self) -> &[String] {
        &self.recipients
    }

    fn push_recipient(&mut self, recipient: impl Into<String>) -> Result<(), SmtpError> {
        let recipient = recipient.into();
        if recipient.trim().is_empty() {
            return Err(SmtpError::EmptyRecipient);
        }
        self.recipients.push(recipient);
        Ok(())
    }
}

/// Encabezados del mensaje de correo electrónico.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EmailHeaders {
    entries: BTreeMap<String, String>,
}

impl EmailHeaders {
    /// Crea encabezados vacíos.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserta un encabezado normalizando el nombre a minúsculas.
    pub fn insert(&mut self, name: impl AsRef<str>, value: impl Into<String>) {
        self.entries
            .insert(name.as_ref().trim().to_ascii_lowercase(), value.into());
    }

    /// Inserta un encabezado y devuelve el mapa para composición.
    pub fn with_header(mut self, name: impl AsRef<str>, value: impl Into<String>) -> Self {
        self.insert(name, value);
        self
    }

    /// Obtiene un encabezado por nombre.
    pub fn get(&self, name: impl AsRef<str>) -> Option<&str> {
        self.entries
            .get(&name.as_ref().trim().to_ascii_lowercase())
            .map(String::as_str)
    }

    /// Itera encabezados en orden estable.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
    }

    fn parse(raw: &str) -> (Self, String) {
        let (head, body) = split_headers_body(raw);
        let mut headers = Self::new();

        for line in head.lines() {
            if let Some((name, value)) = line.split_once(':') {
                if !name.trim().is_empty() {
                    headers.insert(name.trim(), value.trim().to_string());
                }
            }
        }

        (headers, body.to_string())
    }
}

/// Mensaje aceptado por la sesión SMTP educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedMessage {
    envelope: MailEnvelope,
    headers: EmailHeaders,
    body: String,
}

impl AcceptedMessage {
    /// Sobre usado para transportar el mensaje.
    pub fn envelope(&self) -> &MailEnvelope {
        &self.envelope
    }

    /// Encabezados visibles del mensaje.
    pub fn headers(&self) -> &EmailHeaders {
        &self.headers
    }

    /// Cuerpo del mensaje.
    pub fn body(&self) -> &str {
        &self.body
    }
}

/// Registro MX educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MxRecord {
    exchange: String,
    priority: u16,
}

impl MxRecord {
    /// Crea un registro MX con servidor y prioridad.
    pub fn new(exchange: impl Into<String>, priority: u16) -> Self {
        Self {
            exchange: exchange.into(),
            priority,
        }
    }

    /// Servidor de intercambio de correo.
    pub fn exchange(&self) -> &str {
        &self.exchange
    }

    /// Prioridad MX. El menor valor se intenta primero.
    pub fn priority(&self) -> u16 {
        self.priority
    }
}

/// Selecciona el registro MX con menor prioridad numérica.
pub fn select_mx_by_priority(records: &[MxRecord]) -> Option<&MxRecord> {
    records.iter().min_by_key(|record| record.priority())
}

/// Sesión SMTP educativa con validación de orden de comandos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmtpSession {
    server_name: String,
    state: SmtpState,
    current_envelope: Option<MailEnvelope>,
    accepted_messages: Vec<AcceptedMessage>,
}

impl SmtpSession {
    /// Crea una sesión en espera de saludo.
    pub fn new(server_name: impl Into<String>) -> Self {
        Self {
            server_name: server_name.into(),
            state: SmtpState::AwaitHelo,
            current_envelope: None,
            accepted_messages: Vec::new(),
        }
    }

    /// Aplica un comando y devuelve la respuesta SMTP.
    pub fn apply(&mut self, command: SmtpCommand) -> Result<SmtpReply, SmtpError> {
        match (&self.state, command) {
            (SmtpState::AwaitHelo, SmtpCommand::Helo(client)) => {
                self.state = SmtpState::AwaitMailFrom;
                Ok(SmtpReply::new(
                    250,
                    format!("{} saluda a {client}", self.server_name),
                ))
            }
            (SmtpState::AwaitMailFrom, SmtpCommand::MailFrom(sender)) => {
                if sender.trim().is_empty() {
                    return Err(SmtpError::EmptySender);
                }
                self.current_envelope = Some(MailEnvelope::new(sender));
                self.state = SmtpState::AwaitRcptTo;
                Ok(SmtpReply::new(250, "remitente aceptado"))
            }
            (SmtpState::AwaitRcptTo, SmtpCommand::RcptTo(recipient)) => {
                self.envelope_mut()?.push_recipient(recipient)?;
                self.state = SmtpState::AwaitRcptOrData;
                Ok(SmtpReply::new(250, "destinatario aceptado"))
            }
            (SmtpState::AwaitRcptOrData, SmtpCommand::RcptTo(recipient)) => {
                self.envelope_mut()?.push_recipient(recipient)?;
                Ok(SmtpReply::new(250, "destinatario aceptado"))
            }
            (SmtpState::AwaitRcptOrData, SmtpCommand::Data) => {
                self.state = SmtpState::AwaitMessageData;
                Ok(SmtpReply::new(354, "termina datos con <CRLF>.<CRLF>"))
            }
            (SmtpState::AwaitMessageData, SmtpCommand::MessageData(raw)) => {
                let envelope = self
                    .current_envelope
                    .take()
                    .ok_or(SmtpError::MissingEnvelope)?;
                let (headers, body) = EmailHeaders::parse(&raw);
                self.accepted_messages.push(AcceptedMessage {
                    envelope,
                    headers,
                    body,
                });
                self.state = SmtpState::AwaitMailFrom;
                Ok(SmtpReply::new(250, "mensaje aceptado"))
            }
            (_, SmtpCommand::Quit) => {
                self.state = SmtpState::Closed;
                Ok(SmtpReply::new(221, "sesión cerrada"))
            }
            (state, command) => Err(SmtpError::UnexpectedCommand {
                expected: state.expected_command().to_string(),
                actual: command.name().to_string(),
            }),
        }
    }

    /// Nombre del servidor SMTP.
    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    /// Mensajes aceptados en la sesión.
    pub fn accepted_messages(&self) -> &[AcceptedMessage] {
        &self.accepted_messages
    }

    fn envelope_mut(&mut self) -> Result<&mut MailEnvelope, SmtpError> {
        self.current_envelope
            .as_mut()
            .ok_or(SmtpError::MissingEnvelope)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SmtpState {
    AwaitHelo,
    AwaitMailFrom,
    AwaitRcptTo,
    AwaitRcptOrData,
    AwaitMessageData,
    Closed,
}

impl SmtpState {
    fn expected_command(&self) -> &'static str {
        match self {
            Self::AwaitHelo => "HELO",
            Self::AwaitMailFrom => "MAIL FROM",
            Self::AwaitRcptTo => "RCPT TO",
            Self::AwaitRcptOrData => "RCPT TO or DATA",
            Self::AwaitMessageData => "message data",
            Self::Closed => "closed session",
        }
    }
}

/// Error educativo de SMTP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmtpError {
    UnexpectedCommand { expected: String, actual: String },
    MissingEnvelope,
    EmptySender,
    EmptyRecipient,
}

fn split_headers_body(raw: &str) -> (&str, &str) {
    if let Some((head, body)) = raw.split_once("\r\n\r\n") {
        (head, body)
    } else if let Some((head, body)) = raw.split_once("\n\n") {
        (head, body)
    } else {
        ("", raw)
    }
}
