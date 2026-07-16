//! TCP.
//!
//! Objetivo de aprendizaje: entender conexiones, entrega confiable, orden,
//! ventanas, retransmisión y cierre.

use std::collections::BTreeMap;

/// Número de secuencia TCP educativo.
///
/// # Examples
///
/// ```
/// use rust_networking::tcp::SequenceNumber;
///
/// let sequence = SequenceNumber::new(10).advance(5);
/// assert_eq!(sequence.get(), 15);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequenceNumber(u32);

impl SequenceNumber {
    /// Crea un número de secuencia.
    ///
    /// Complejidad: O(1).
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    ///
    /// Complejidad: O(1).
    pub fn get(self) -> u32 {
        self.0
    }

    /// Avanza el número de secuencia.
    ///
    /// Complejidad: O(1).
    pub fn advance(self, amount: usize) -> Self {
        Self(self.0 + amount as u32)
    }
}

/// Estado principal de una conexión TCP educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    /// No hay conexión abierta.
    Closed,
    /// El cliente envió `SYN` y espera `SYN+ACK`.
    SynSent,
    /// El servidor recibió `SYN`, envió `SYN+ACK` y espera `ACK`.
    SynReceived,
    /// Ambos lados consideran establecida la conexión.
    Established,
    /// Se envió `FIN` y se espera su acuse.
    FinWait1,
}

/// Evento observable del modelo TCP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpEvent {
    /// Se envió un `SYN`.
    SynSent,
    /// Se recibió un `SYN`.
    SynReceived,
    /// La conexión llegó a `Established`.
    ConnectionEstablished,
    /// Un segmento llegó antes de tiempo y quedó en buffer.
    DataBuffered,
    /// Datos contiguos fueron entregados a la aplicación.
    DataDelivered,
    /// Hay segmentos no reconocidos listos para retransmitirse.
    RetransmissionScheduled,
    /// Se envió `FIN`.
    FinSent,
    /// La conexión cerró de forma ordenada en este modelo.
    ConnectionClosed,
}

/// Error educativo del modelo TCP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpError {
    /// El segmento no es válido para el estado actual.
    UnexpectedSegment { state: TcpState, flags: String },
    /// La operación requiere una conexión establecida.
    NotEstablished { state: TcpState },
    /// No hay segmentos pendientes de acuse.
    NothingToRetransmit,
}

/// Segmento TCP educativo.
///
/// Este tipo no serializa encabezados reales. Representa banderas, número de
/// secuencia, acuse y carga útil para enseñar el protocolo.
///
/// # Examples
///
/// ```
/// use rust_networking::tcp::{SequenceNumber, TcpSegment};
///
/// let segment = TcpSegment::syn(SequenceNumber::new(1));
/// assert!(segment.is_syn());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpSegment {
    sequence_number: SequenceNumber,
    ack_number: Option<SequenceNumber>,
    syn: bool,
    ack: bool,
    fin: bool,
    payload: Vec<u8>,
}

impl TcpSegment {
    /// Crea un segmento `SYN`.
    ///
    /// Complejidad: O(1).
    pub fn syn(sequence_number: SequenceNumber) -> Self {
        Self::new(sequence_number, None, true, false, false, Vec::new())
    }

    /// Crea un segmento `SYN+ACK`.
    ///
    /// Complejidad: O(1).
    pub fn syn_ack(sequence_number: SequenceNumber, ack_number: SequenceNumber) -> Self {
        Self::new(
            sequence_number,
            Some(ack_number),
            true,
            true,
            false,
            Vec::new(),
        )
    }

    /// Crea un segmento `ACK`.
    ///
    /// Complejidad: O(1).
    pub fn ack(sequence_number: SequenceNumber, ack_number: SequenceNumber) -> Self {
        Self::new(
            sequence_number,
            Some(ack_number),
            false,
            true,
            false,
            Vec::new(),
        )
    }

    /// Crea un segmento `FIN`.
    ///
    /// Complejidad: O(1).
    pub fn fin(sequence_number: SequenceNumber) -> Self {
        Self::new(sequence_number, None, false, false, true, Vec::new())
    }

    /// Crea un segmento de datos.
    ///
    /// Complejidad: O(1) más la propiedad del vector recibido.
    pub fn data(sequence_number: SequenceNumber, payload: Vec<u8>) -> Self {
        Self::new(sequence_number, None, false, false, false, payload)
    }

    fn new(
        sequence_number: SequenceNumber,
        ack_number: Option<SequenceNumber>,
        syn: bool,
        ack: bool,
        fin: bool,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            sequence_number,
            ack_number,
            syn,
            ack,
            fin,
            payload,
        }
    }

    /// Devuelve el número de secuencia.
    ///
    /// Complejidad: O(1).
    pub fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }

    /// Devuelve el número de acuse, si existe.
    ///
    /// Complejidad: O(1).
    pub fn ack_number(&self) -> Option<SequenceNumber> {
        self.ack_number
    }

    /// Indica si el segmento tiene bandera `SYN`.
    ///
    /// Complejidad: O(1).
    pub fn is_syn(&self) -> bool {
        self.syn
    }

    /// Indica si el segmento tiene bandera `ACK`.
    ///
    /// Complejidad: O(1).
    pub fn is_ack(&self) -> bool {
        self.ack
    }

    /// Indica si el segmento tiene bandera `FIN`.
    ///
    /// Complejidad: O(1).
    pub fn is_fin(&self) -> bool {
        self.fin
    }

    /// Devuelve la carga útil.
    ///
    /// Complejidad: O(1).
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    fn consumes_sequence_space(&self) -> usize {
        self.payload.len() + usize::from(self.syn) + usize::from(self.fin)
    }

    fn end_sequence_number(&self) -> SequenceNumber {
        self.sequence_number.advance(self.consumes_sequence_space())
    }

    fn flag_summary(&self) -> String {
        let mut flags = Vec::new();
        if self.syn {
            flags.push("SYN");
        }
        if self.ack {
            flags.push("ACK");
        }
        if self.fin {
            flags.push("FIN");
        }
        if flags.is_empty() && !self.payload.is_empty() {
            flags.push("DATA");
        }
        if flags.is_empty() {
            flags.push("EMPTY");
        }
        flags.join("+")
    }
}

/// Conexión TCP educativa.
///
/// # Examples
///
/// ```
/// use rust_networking::tcp::{SequenceNumber, TcpConnection, TcpState};
///
/// let mut cliente = TcpConnection::new_client(SequenceNumber::new(1));
/// let syn = cliente.open().unwrap();
/// assert!(syn.is_syn());
/// assert_eq!(cliente.state(), TcpState::SynSent);
/// ```
#[derive(Debug, Clone)]
pub struct TcpConnection {
    state: TcpState,
    send_next: SequenceNumber,
    receive_next: SequenceNumber,
    unacked: Vec<TcpSegment>,
    pending: BTreeMap<SequenceNumber, Vec<u8>>,
    received_payload: Vec<u8>,
    events: Vec<TcpEvent>,
}

impl TcpConnection {
    /// Crea una conexión cliente cerrada.
    ///
    /// Complejidad: O(1).
    pub fn new_client(initial_sequence: SequenceNumber) -> Self {
        Self::closed(initial_sequence)
    }

    /// Crea una conexión servidor cerrada.
    ///
    /// Complejidad: O(1).
    pub fn new_server(initial_sequence: SequenceNumber) -> Self {
        Self::closed(initial_sequence)
    }

    /// Crea una conexión ya establecida para ejemplos y pruebas de flujo.
    ///
    /// Complejidad: O(1).
    pub fn established(send_next: SequenceNumber, receive_next: SequenceNumber) -> Self {
        Self {
            state: TcpState::Established,
            send_next,
            receive_next,
            unacked: Vec::new(),
            pending: BTreeMap::new(),
            received_payload: Vec::new(),
            events: vec![TcpEvent::ConnectionEstablished],
        }
    }

    fn closed(initial_sequence: SequenceNumber) -> Self {
        Self {
            state: TcpState::Closed,
            send_next: initial_sequence,
            receive_next: SequenceNumber::new(0),
            unacked: Vec::new(),
            pending: BTreeMap::new(),
            received_payload: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Devuelve el estado actual.
    ///
    /// Complejidad: O(1).
    pub fn state(&self) -> TcpState {
        self.state
    }

    /// Devuelve eventos observados.
    ///
    /// Complejidad: O(1).
    pub fn events(&self) -> &[TcpEvent] {
        &self.events
    }

    /// Inicia un establecimiento de conexión desde el cliente.
    ///
    /// Complejidad: O(1).
    pub fn open(&mut self) -> Result<TcpSegment, TcpError> {
        if self.state != TcpState::Closed {
            return Err(TcpError::UnexpectedSegment {
                state: self.state,
                flags: "OPEN".to_string(),
            });
        }

        let segment = TcpSegment::syn(self.send_next);
        self.send_next = self.send_next.advance(1);
        self.state = TcpState::SynSent;
        self.unacked.push(segment.clone());
        self.events.push(TcpEvent::SynSent);
        Ok(segment)
    }

    /// Recibe un segmento y devuelve una respuesta si el protocolo la requiere.
    ///
    /// Complejidad: O(p log p), donde `p` es el número de segmentos pendientes.
    pub fn receive(&mut self, segment: TcpSegment) -> Result<Option<TcpSegment>, TcpError> {
        match self.state {
            TcpState::Closed => self.receive_when_closed(segment),
            TcpState::SynSent => self.receive_when_syn_sent(segment),
            TcpState::SynReceived => self.receive_when_syn_received(segment),
            TcpState::Established => self.receive_when_established(segment),
            TcpState::FinWait1 => self.receive_when_fin_wait_1(segment),
        }
    }

    fn receive_when_closed(&mut self, segment: TcpSegment) -> Result<Option<TcpSegment>, TcpError> {
        if segment.is_syn() && !segment.is_ack() {
            self.receive_next = segment.sequence_number().advance(1);
            let respuesta = TcpSegment::syn_ack(self.send_next, self.receive_next);
            self.send_next = self.send_next.advance(1);
            self.state = TcpState::SynReceived;
            self.unacked.push(respuesta.clone());
            self.events.push(TcpEvent::SynReceived);
            Ok(Some(respuesta))
        } else {
            Err(unexpected(self.state, &segment))
        }
    }

    fn receive_when_syn_sent(
        &mut self,
        segment: TcpSegment,
    ) -> Result<Option<TcpSegment>, TcpError> {
        if segment.is_syn() && segment.is_ack() && segment.ack_number() == Some(self.send_next) {
            self.clear_acked(segment.ack_number().unwrap());
            self.receive_next = segment.sequence_number().advance(1);
            let respuesta = TcpSegment::ack(self.send_next, self.receive_next);
            self.state = TcpState::Established;
            self.events.push(TcpEvent::ConnectionEstablished);
            Ok(Some(respuesta))
        } else {
            Err(unexpected(self.state, &segment))
        }
    }

    fn receive_when_syn_received(
        &mut self,
        segment: TcpSegment,
    ) -> Result<Option<TcpSegment>, TcpError> {
        if segment.is_ack() && segment.ack_number() == Some(self.send_next) {
            self.clear_acked(segment.ack_number().unwrap());
            self.state = TcpState::Established;
            self.events.push(TcpEvent::ConnectionEstablished);
            Ok(None)
        } else {
            Err(unexpected(self.state, &segment))
        }
    }

    fn receive_when_established(
        &mut self,
        segment: TcpSegment,
    ) -> Result<Option<TcpSegment>, TcpError> {
        if segment.is_ack() && segment.payload().is_empty() && !segment.is_fin() {
            if let Some(ack_number) = segment.ack_number() {
                self.clear_acked(ack_number);
            }
            return Ok(None);
        }

        if segment.is_fin() {
            self.receive_next = segment.sequence_number().advance(1);
            self.state = TcpState::Closed;
            self.events.push(TcpEvent::ConnectionClosed);
            return Ok(Some(TcpSegment::ack(self.send_next, self.receive_next)));
        }

        if !segment.payload().is_empty() {
            self.receive_data(segment);
            return Ok(Some(TcpSegment::ack(self.send_next, self.receive_next)));
        }

        Err(unexpected(self.state, &segment))
    }

    fn receive_when_fin_wait_1(
        &mut self,
        segment: TcpSegment,
    ) -> Result<Option<TcpSegment>, TcpError> {
        if segment.is_ack() && segment.ack_number() == Some(self.send_next) {
            self.clear_acked(segment.ack_number().unwrap());
            self.state = TcpState::Closed;
            self.events.push(TcpEvent::ConnectionClosed);
            Ok(None)
        } else {
            Err(unexpected(self.state, &segment))
        }
    }

    fn receive_data(&mut self, segment: TcpSegment) {
        if segment.sequence_number() == self.receive_next {
            self.deliver_payload(segment.payload());
            self.receive_next = segment.end_sequence_number();
            self.events.push(TcpEvent::DataDelivered);
            self.drain_pending_segments();
        } else if segment.sequence_number() > self.receive_next {
            self.pending
                .insert(segment.sequence_number(), segment.payload().to_vec());
            self.events.push(TcpEvent::DataBuffered);
        }
    }

    fn drain_pending_segments(&mut self) {
        while let Some(payload) = self.pending.remove(&self.receive_next) {
            self.receive_next = self.receive_next.advance(payload.len());
            self.deliver_payload(&payload);
            self.events.push(TcpEvent::DataDelivered);
        }
    }

    fn deliver_payload(&mut self, payload: &[u8]) {
        self.received_payload.extend_from_slice(payload);
    }

    fn clear_acked(&mut self, ack_number: SequenceNumber) {
        self.unacked
            .retain(|segment| segment.end_sequence_number() > ack_number);
    }

    /// Devuelve el número de segmentos fuera de orden pendientes.
    ///
    /// Complejidad: O(1).
    pub fn pending_segments(&self) -> usize {
        self.pending.len()
    }

    /// Devuelve la carga útil entregada a la aplicación.
    ///
    /// Complejidad: O(1).
    pub fn received_payload(&self) -> &[u8] {
        &self.received_payload
    }

    /// Devuelve segmentos enviados que aún no tienen acuse.
    ///
    /// Complejidad: O(1).
    pub fn unacked_segments(&self) -> &[TcpSegment] {
        &self.unacked
    }

    /// Envía datos desde una conexión establecida.
    ///
    /// Complejidad: O(1) más la propiedad del vector recibido.
    pub fn send_data(&mut self, payload: Vec<u8>) -> Result<TcpSegment, TcpError> {
        if self.state != TcpState::Established {
            return Err(TcpError::NotEstablished { state: self.state });
        }

        let segment = TcpSegment::data(self.send_next, payload);
        self.send_next = segment.end_sequence_number();
        self.unacked.push(segment.clone());
        Ok(segment)
    }

    /// Devuelve una copia de los segmentos que deben retransmitirse.
    ///
    /// Complejidad: O(n).
    pub fn retransmit_unacked(&mut self) -> Result<Vec<TcpSegment>, TcpError> {
        if self.unacked.is_empty() {
            return Err(TcpError::NothingToRetransmit);
        }

        self.events.push(TcpEvent::RetransmissionScheduled);
        Ok(self.unacked.clone())
    }

    /// Inicia un cierre ordenado con `FIN`.
    ///
    /// Complejidad: O(1).
    pub fn close(&mut self) -> Result<TcpSegment, TcpError> {
        if self.state != TcpState::Established {
            return Err(TcpError::NotEstablished { state: self.state });
        }

        let segment = TcpSegment::fin(self.send_next);
        self.send_next = self.send_next.advance(1);
        self.state = TcpState::FinWait1;
        self.unacked.push(segment.clone());
        self.events.push(TcpEvent::FinSent);
        Ok(segment)
    }
}

fn unexpected(state: TcpState, segment: &TcpSegment) -> TcpError {
    TcpError::UnexpectedSegment {
        state,
        flags: segment.flag_summary(),
    }
}
