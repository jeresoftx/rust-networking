//! UDP.
//!
//! Objetivo de aprendizaje: entender datagramas, ausencia de conexión, pérdida,
//! duplicación y cuándo una abstracción mínima es suficiente.

/// Extremo UDP educativo.
///
/// # Examples
///
/// ```
/// use rust_networking::udp::UdpEndpoint;
///
/// let endpoint = UdpEndpoint::new("127.0.0.1", 8080);
/// assert_eq!(endpoint.port(), 8080);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UdpEndpoint {
    address: String,
    port: u16,
}

impl UdpEndpoint {
    /// Crea un extremo con dirección textual y puerto.
    ///
    /// Complejidad: O(a), donde `a` es la longitud de la dirección.
    pub fn new(address: impl Into<String>, port: u16) -> Self {
        Self {
            address: address.into(),
            port,
        }
    }

    /// Devuelve la dirección textual.
    ///
    /// Complejidad: O(1).
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Devuelve el puerto.
    ///
    /// Complejidad: O(1).
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Error educativo del modelo UDP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UdpError {
    /// La carga útil excede el máximo de UDP sobre IPv4 sin opciones.
    PayloadTooLarge { size: usize, max: usize },
}

/// Datagrama UDP educativo.
///
/// Este tipo representa el contrato básico de UDP: origen, destino y carga
/// útil independiente. No modela encabezados reales ni checksum.
///
/// # Examples
///
/// ```
/// use rust_networking::udp::{UdpDatagram, UdpEndpoint};
///
/// let source = UdpEndpoint::new("sensor", 4000);
/// let destination = UdpEndpoint::new("collector", 8125);
/// let datagram = UdpDatagram::new(source, destination, b"ok".to_vec()).unwrap();
///
/// assert_eq!(datagram.payload(), b"ok");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpDatagram {
    source: UdpEndpoint,
    destination: UdpEndpoint,
    payload: Vec<u8>,
}

impl UdpDatagram {
    /// Tamaño máximo práctico de carga útil UDP sobre IPv4 sin opciones.
    pub const MAX_PAYLOAD_SIZE: usize = 65_507;

    /// Crea un datagrama y valida el tamaño de la carga útil.
    ///
    /// Complejidad: O(1) más la propiedad del vector recibido.
    pub fn new(
        source: UdpEndpoint,
        destination: UdpEndpoint,
        payload: Vec<u8>,
    ) -> Result<Self, UdpError> {
        if payload.len() > Self::MAX_PAYLOAD_SIZE {
            return Err(UdpError::PayloadTooLarge {
                size: payload.len(),
                max: Self::MAX_PAYLOAD_SIZE,
            });
        }

        Ok(Self {
            source,
            destination,
            payload,
        })
    }

    /// Devuelve el origen.
    ///
    /// Complejidad: O(1).
    pub fn source(&self) -> &UdpEndpoint {
        &self.source
    }

    /// Devuelve el destino.
    ///
    /// Complejidad: O(1).
    pub fn destination(&self) -> &UdpEndpoint {
        &self.destination
    }

    /// Devuelve la carga útil.
    ///
    /// Complejidad: O(1).
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Devuelve el tamaño de la carga útil.
    ///
    /// Complejidad: O(1).
    pub fn len(&self) -> usize {
        self.payload.len()
    }

    /// Indica si la carga útil está vacía.
    ///
    /// Complejidad: O(1).
    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

/// Resultado de entrega de mejor esfuerzo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryOutcome {
    /// El datagrama llegó una vez.
    Delivered(UdpDatagram),
    /// El datagrama se perdió.
    Lost,
    /// El datagrama llegó más de una vez.
    Duplicated(Vec<UdpDatagram>),
}

impl DeliveryOutcome {
    /// Simula una entrega determinista para pruebas y ejemplos.
    ///
    /// Múltiplos de 5 se pierden, múltiplos de 3 se duplican y el resto se
    /// entrega una vez. Cuando una secuencia cumple ambas reglas, gana pérdida.
    ///
    /// Complejidad: O(k) cuando duplica, donde `k` es el tamaño del datagrama.
    pub fn deterministic(datagram: UdpDatagram, sequence: u64) -> Self {
        if sequence.is_multiple_of(5) {
            Self::Lost
        } else if sequence.is_multiple_of(3) {
            Self::Duplicated(vec![datagram.clone(), datagram])
        } else {
            Self::Delivered(datagram)
        }
    }
}
