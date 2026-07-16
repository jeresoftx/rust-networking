//! Modelo de capas, IP y enrutamiento.
//!
//! Objetivo de aprendizaje: entender cómo se encapsulan datos, qué papel juega
//! IP, cómo se decide una ruta y por qué la red base entrega paquetes con
//! garantías limitadas.

use std::fmt;

/// Capa de red en el modelo TCP/IP pragmático del curso.
///
/// # Examples
///
/// ```
/// use rust_networking::layers::NetworkLayer;
///
/// assert_eq!(NetworkLayer::Application.as_str(), "Application");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLayer {
    /// Protocolos de aplicación: HTTP, DNS, SMTP, gRPC.
    Application,
    /// Transporte extremo a extremo: TCP, UDP, QUIC.
    Transport,
    /// Entrega entre redes: IP y enrutamiento.
    Internet,
    /// Entrega local en el enlace: Ethernet, Wi-Fi.
    Link,
}

impl NetworkLayer {
    /// Devuelve el nombre canónico usado en diagramas del curso.
    ///
    /// Complejidad: O(1).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Application => "Application",
            Self::Transport => "Transport",
            Self::Internet => "Internet",
            Self::Link => "Link",
        }
    }
}

impl fmt::Display for NetworkLayer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error educativo al decrementar TTL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtlError {
    /// El paquete ya no debe reenviarse porque su TTL llegó a cero.
    Expired,
}

/// Representa datos encapsulados al atravesar capas.
///
/// `EncapsulatedFrame` no serializa bytes reales. Conserva el payload original
/// y una traza de capas para enseñar el orden de encapsulación.
///
/// # Examples
///
/// ```
/// use rust_networking::layers::{EncapsulatedFrame, NetworkLayer};
///
/// let frame = EncapsulatedFrame::new("hello")
///     .wrap(NetworkLayer::Application, "HTTP")
///     .wrap(NetworkLayer::Transport, "TCP");
///
/// assert_eq!(frame.describe_path(), "Application -> Transport");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncapsulatedFrame {
    payload: String,
    wrappers: Vec<LayerWrapper>,
    ttl: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LayerWrapper {
    layer: NetworkLayer,
    label: String,
}

impl EncapsulatedFrame {
    /// Crea un frame educativo con TTL inicial de 64.
    ///
    /// Complejidad: O(n), donde `n` es el tamaño del payload.
    pub fn new(payload: impl Into<String>) -> Self {
        Self {
            payload: payload.into(),
            wrappers: Vec::new(),
            ttl: 64,
        }
    }

    /// Ajusta el TTL del frame.
    ///
    /// Complejidad: O(1).
    pub fn with_ttl(mut self, ttl: u8) -> Self {
        self.ttl = ttl;
        self
    }

    /// Envuelve el frame con una capa nueva.
    ///
    /// Complejidad: O(1) más la copia de la etiqueta.
    pub fn wrap(mut self, layer: NetworkLayer, label: impl Into<String>) -> Self {
        self.wrappers.push(LayerWrapper {
            layer,
            label: label.into(),
        });
        self
    }

    /// Devuelve el payload original.
    ///
    /// Complejidad: O(1).
    pub fn payload(&self) -> &str {
        &self.payload
    }

    /// Devuelve las capas en orden de adentro hacia afuera.
    ///
    /// Complejidad: O(n).
    pub fn layers(&self) -> Vec<NetworkLayer> {
        self.wrappers.iter().map(|wrapper| wrapper.layer).collect()
    }

    /// Devuelve la capa más externa.
    ///
    /// Complejidad: O(1).
    pub fn outermost_layer(&self) -> Option<NetworkLayer> {
        self.wrappers.last().map(|wrapper| wrapper.layer)
    }

    /// Describe el camino de encapsulación.
    ///
    /// Complejidad: O(n).
    pub fn describe_path(&self) -> String {
        self.wrappers
            .iter()
            .map(|wrapper| wrapper.layer.as_str())
            .collect::<Vec<_>>()
            .join(" -> ")
    }

    /// Devuelve las etiquetas asociadas a cada capa.
    ///
    /// Complejidad: O(n).
    pub fn labels(&self) -> Vec<&str> {
        self.wrappers
            .iter()
            .map(|wrapper| wrapper.label.as_str())
            .collect()
    }

    /// Devuelve el TTL actual.
    ///
    /// Complejidad: O(1).
    pub fn ttl(&self) -> u8 {
        self.ttl
    }

    /// Decrementa TTL antes de reenviar.
    ///
    /// Si el TTL llega a cero, el paquete debe descartarse.
    ///
    /// Complejidad: O(1).
    pub fn decrement_ttl(&mut self) -> Result<u8, TtlError> {
        self.ttl = self.ttl.saturating_sub(1);

        if self.ttl == 0 {
            Err(TtlError::Expired)
        } else {
            Ok(self.ttl)
        }
    }
}

/// Dirección IPv4 representada como cuatro octetos.
///
/// # Examples
///
/// ```
/// use rust_networking::layers::Ipv4Address;
///
/// let address = Ipv4Address::new(192, 168, 1, 10);
/// assert_eq!(address.to_string(), "192.168.1.10");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ipv4Address {
    octets: [u8; 4],
}

impl Ipv4Address {
    /// Crea una dirección IPv4.
    ///
    /// Complejidad: O(1).
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            octets: [a, b, c, d],
        }
    }

    /// Devuelve los cuatro octetos.
    ///
    /// Complejidad: O(1).
    pub fn octets(self) -> [u8; 4] {
        self.octets
    }

    /// Convierte la dirección a entero para aplicar máscaras.
    ///
    /// Complejidad: O(1).
    pub fn to_u32(self) -> u32 {
        u32::from_be_bytes(self.octets)
    }
}

impl fmt::Display for Ipv4Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}.{}",
            self.octets[0], self.octets[1], self.octets[2], self.octets[3]
        )
    }
}

/// Rango IPv4 en notación CIDR.
///
/// # Examples
///
/// ```
/// use rust_networking::layers::{Ipv4Address, Ipv4Cidr};
///
/// let cidr = Ipv4Cidr::new(Ipv4Address::new(10, 0, 0, 0), 8).unwrap();
/// assert!(cidr.contains(Ipv4Address::new(10, 20, 30, 40)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Cidr {
    network: Ipv4Address,
    prefix_len: u8,
}

impl Ipv4Cidr {
    /// Crea un rango CIDR si el prefijo está entre 0 y 32.
    ///
    /// Complejidad: O(1).
    pub fn new(network: Ipv4Address, prefix_len: u8) -> Result<Self, CidrError> {
        if prefix_len > 32 {
            return Err(CidrError::InvalidPrefixLength(prefix_len));
        }

        Ok(Self {
            network,
            prefix_len,
        })
    }

    /// Devuelve la red base.
    ///
    /// Complejidad: O(1).
    pub fn network(self) -> Ipv4Address {
        self.network
    }

    /// Devuelve la longitud del prefijo.
    ///
    /// Complejidad: O(1).
    pub fn prefix_len(self) -> u8 {
        self.prefix_len
    }

    /// Indica si `address` cae dentro del rango.
    ///
    /// Complejidad: O(1).
    pub fn contains(self, address: Ipv4Address) -> bool {
        let mask = prefix_mask(self.prefix_len);
        self.network.to_u32() & mask == address.to_u32() & mask
    }
}

impl fmt::Display for Ipv4Cidr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.network, self.prefix_len)
    }
}

/// Error al construir un CIDR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CidrError {
    /// El prefijo IPv4 debe estar entre 0 y 32.
    InvalidPrefixLength(u8),
}

/// Ruta candidata dentro de una tabla.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    destination: Ipv4Cidr,
    interface: String,
    next_hop: Option<Ipv4Address>,
}

impl Route {
    /// Crea una ruta.
    ///
    /// Complejidad: O(n), donde `n` es el tamaño del nombre de interfaz.
    pub fn new(
        destination: Ipv4Cidr,
        interface: impl Into<String>,
        next_hop: Option<Ipv4Address>,
    ) -> Self {
        Self {
            destination,
            interface: interface.into(),
            next_hop,
        }
    }

    /// Devuelve el destino CIDR.
    ///
    /// Complejidad: O(1).
    pub fn destination(&self) -> Ipv4Cidr {
        self.destination
    }

    /// Devuelve la interfaz de salida.
    ///
    /// Complejidad: O(1).
    pub fn interface(&self) -> &str {
        &self.interface
    }

    /// Devuelve el siguiente salto, si aplica.
    ///
    /// Complejidad: O(1).
    pub fn next_hop(&self) -> Option<Ipv4Address> {
        self.next_hop
    }
}

/// Decisión tomada por una tabla de rutas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDecision {
    /// Destino consultado.
    pub destination: Ipv4Address,
    /// Interfaz de salida.
    pub interface: String,
    /// Siguiente salto cuando la ruta no es directamente conectada.
    pub next_hop: Option<Ipv4Address>,
    /// Prefijo que ganó por ser el más específico.
    pub matched_prefix: Ipv4Cidr,
}

/// Tabla de rutas educativa.
///
/// # Examples
///
/// ```
/// use rust_networking::layers::{Ipv4Address, Ipv4Cidr, Route, RoutingTable};
///
/// let table = RoutingTable::new(vec![
///     Route::new(Ipv4Cidr::new(Ipv4Address::new(0, 0, 0, 0), 0).unwrap(), "wan", None),
/// ]);
///
/// assert!(table.select_route(Ipv4Address::new(8, 8, 8, 8)).is_some());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingTable {
    routes: Vec<Route>,
}

impl RoutingTable {
    /// Crea una tabla de rutas en el orden recibido.
    ///
    /// Complejidad: O(1).
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }

    /// Devuelve las rutas registradas.
    ///
    /// Complejidad: O(1).
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    /// Selecciona la ruta con el prefijo más específico.
    ///
    /// Complejidad: O(r), donde `r` es el número de rutas.
    pub fn select_route(&self, destination: Ipv4Address) -> Option<RouteDecision> {
        self.routes
            .iter()
            .filter(|route| route.destination.contains(destination))
            .max_by_key(|route| route.destination.prefix_len())
            .map(|route| RouteDecision {
                destination,
                interface: route.interface.clone(),
                next_hop: route.next_hop,
                matched_prefix: route.destination,
            })
    }
}

fn prefix_mask(prefix_len: u8) -> u32 {
    if prefix_len == 0 {
        0
    } else {
        u32::MAX << (32 - prefix_len)
    }
}
