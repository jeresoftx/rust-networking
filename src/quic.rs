//! QUIC.
//!
//! Objetivo de aprendizaje: entender transporte sobre UDP, flujos,
//! negociación integrada, migración de conexión y relación con HTTP/3.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de conexión QUIC.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuicConnectionId(String);

impl QuicConnectionId {
    /// Crea un identificador no vacío.
    pub fn new(value: impl Into<String>) -> Result<Self, QuicError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(QuicError::EmptyConnectionId);
        }

        Ok(Self(value))
    }

    /// Devuelve el identificador textual.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identificador de flujo QUIC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuicStreamId(u64);

impl QuicStreamId {
    /// Crea un identificador de flujo educativo.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Paquete QUIC educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuicPacket {
    connection_id: QuicConnectionId,
    stream_id: QuicStreamId,
    sequence: u64,
    payload: Vec<u8>,
}

impl QuicPacket {
    /// Crea un paquete asociado a una conexión y a un flujo.
    pub fn new(
        connection_id: QuicConnectionId,
        stream_id: QuicStreamId,
        sequence: u64,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            connection_id,
            stream_id,
            sequence,
            payload,
        }
    }

    /// Identidad de conexión.
    pub fn connection_id(&self) -> &QuicConnectionId {
        &self.connection_id
    }

    /// Flujo al que pertenece el paquete.
    pub fn stream_id(&self) -> QuicStreamId {
        self.stream_id
    }

    /// Secuencia dentro del flujo.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Carga útil del paquete.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Flujo QUIC educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuicStream {
    id: QuicStreamId,
    received: BTreeMap<u64, Vec<u8>>,
    lost: BTreeSet<u64>,
}

impl QuicStream {
    /// Crea un flujo sin paquetes.
    pub fn new(id: QuicStreamId) -> Self {
        Self {
            id,
            received: BTreeMap::new(),
            lost: BTreeSet::new(),
        }
    }

    /// Identificador del flujo.
    pub fn id(&self) -> QuicStreamId {
        self.id
    }

    /// Recibe un paquete si pertenece al flujo.
    pub fn receive(&mut self, packet: QuicPacket) -> Result<(), QuicError> {
        if packet.stream_id() != self.id {
            return Err(QuicError::UnexpectedStream {
                expected: self.id,
                actual: packet.stream_id(),
            });
        }

        self.lost.remove(&packet.sequence());
        self.received.insert(packet.sequence(), packet.payload);
        Ok(())
    }

    /// Registra pérdida de una secuencia dentro de este flujo.
    pub fn mark_lost(&mut self, sequence: u64) {
        self.lost.insert(sequence);
    }

    /// Indica si este flujo espera retransmisión.
    pub fn is_blocked_by_loss(&self) -> bool {
        !self.lost.is_empty()
    }

    /// Devuelve cargas útiles recibidas en orden de secuencia.
    pub fn payloads_in_order(&self) -> Vec<&[u8]> {
        self.received.values().map(Vec::as_slice).collect()
    }
}

/// Migración de conexión QUIC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionMigration {
    connection_id: QuicConnectionId,
    previous_path: String,
    current_path: String,
    reason: String,
}

impl ConnectionMigration {
    /// Registra un cambio de ruta que conserva el identificador de conexión.
    pub fn new(
        connection_id: QuicConnectionId,
        previous_path: impl Into<String>,
        current_path: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            connection_id,
            previous_path: previous_path.into(),
            current_path: current_path.into(),
            reason: reason.into(),
        }
    }

    /// Identificador preservado.
    pub fn connection_id(&self) -> &QuicConnectionId {
        &self.connection_id
    }

    /// Ruta anterior.
    pub fn previous_path(&self) -> &str {
        &self.previous_path
    }

    /// Ruta actual.
    pub fn current_path(&self) -> &str {
        &self.current_path
    }

    /// Motivo de la migración.
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// En QUIC, la identidad lógica no depende de la dirección IP visible.
    pub fn preserves_connection_identity(&self) -> bool {
        self.previous_path != self.current_path
    }
}

/// Negociación QUIC educativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuicHandshake {
    connection_id: QuicConnectionId,
    tls_version: String,
    application_protocol: String,
}

impl QuicHandshake {
    /// Negocia seguridad integrada y protocolo de aplicación.
    pub fn negotiate(
        connection_id: QuicConnectionId,
        tls_version: impl Into<String>,
        application_protocol: impl Into<String>,
    ) -> Result<Self, QuicError> {
        let tls_version = tls_version.into();
        if tls_version != "TLS 1.3" {
            return Err(QuicError::InsecureHandshake { tls_version });
        }

        let application_protocol = application_protocol.into();
        if application_protocol != "h3" {
            return Err(QuicError::UnsupportedApplicationProtocol {
                application_protocol,
            });
        }

        Ok(Self {
            connection_id,
            tls_version,
            application_protocol,
        })
    }

    /// Identidad de conexión negociada.
    pub fn connection_id(&self) -> &QuicConnectionId {
        &self.connection_id
    }

    /// Versión TLS integrada.
    pub fn tls_version(&self) -> &str {
        &self.tls_version
    }

    /// Protocolo de aplicación negociado.
    pub fn application_protocol(&self) -> &str {
        &self.application_protocol
    }

    /// QUIC moderno exige TLS 1.3 integrado.
    pub fn is_secure(&self) -> bool {
        self.tls_version == "TLS 1.3"
    }
}

/// Error educativo de QUIC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuicError {
    EmptyConnectionId,
    UnexpectedStream {
        expected: QuicStreamId,
        actual: QuicStreamId,
    },
    InsecureHandshake {
        tls_version: String,
    },
    UnsupportedApplicationProtocol {
        application_protocol: String,
    },
}
