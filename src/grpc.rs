//! gRPC.
//!
//! Objetivo de aprendizaje: entender contratos, HTTP/2, flujos, códigos de
//! estado y compatibilidad.

use std::collections::BTreeMap;

/// Modo de flujo de un método gRPC educativo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamMode {
    Unary,
    ServerStreaming,
    ClientStreaming,
    BidirectionalStreaming,
}

impl StreamMode {
    /// Indica si el método es una llamada unaria.
    pub fn is_unary(self) -> bool {
        matches!(self, Self::Unary)
    }

    /// Indica si el cliente puede enviar múltiples mensajes.
    pub fn has_client_stream(self) -> bool {
        matches!(self, Self::ClientStreaming | Self::BidirectionalStreaming)
    }

    /// Indica si el servidor puede enviar múltiples mensajes.
    pub fn has_server_stream(self) -> bool {
        matches!(self, Self::ServerStreaming | Self::BidirectionalStreaming)
    }
}

/// Método gRPC educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrpcMethod {
    name: String,
    version: u32,
    mode: StreamMode,
}

impl GrpcMethod {
    /// Crea un método con nombre, versión de contrato y modo de flujo.
    pub fn new(name: impl Into<String>, version: u32, mode: StreamMode) -> Self {
        Self {
            name: name.into(),
            version,
            mode,
        }
    }

    /// Nombre del método.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Versión de contrato esperada por el método.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Modo de flujo.
    pub fn mode(&self) -> StreamMode {
        self.mode
    }
}

/// Servicio gRPC educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrpcService {
    name: String,
    version: u32,
    methods: BTreeMap<String, GrpcMethod>,
}

impl GrpcService {
    /// Crea un servicio sin métodos registrados.
    pub fn new(name: impl Into<String>, version: u32) -> Self {
        Self {
            name: name.into(),
            version,
            methods: BTreeMap::new(),
        }
    }

    /// Registra un método.
    pub fn add_method(mut self, method: GrpcMethod) -> Result<Self, GrpcError> {
        if self.methods.contains_key(method.name()) {
            return Err(GrpcError::DuplicateMethod(method.name().to_string()));
        }

        self.methods.insert(method.name().to_string(), method);
        Ok(self)
    }

    /// Nombre del servicio.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Versión del servicio.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Busca un método por nombre.
    pub fn method(&self, name: &str) -> Result<&GrpcMethod, GrpcError> {
        self.methods
            .get(name)
            .ok_or_else(|| GrpcError::UnknownMethod(name.to_string()))
    }

    /// Valida compatibilidad de versión para un método.
    pub fn ensure_compatible(
        &self,
        method_name: &str,
        expected_version: u32,
    ) -> Result<(), GrpcError> {
        let method = self.method(method_name)?;
        if method.version() == expected_version {
            Ok(())
        } else {
            Err(GrpcError::IncompatibleVersion {
                method: method.name().to_string(),
                expected: expected_version,
                actual: method.version(),
            })
        }
    }

    /// Itera métodos registrados.
    pub fn methods(&self) -> impl Iterator<Item = &GrpcMethod> {
        self.methods.values()
    }
}

/// Mensaje gRPC educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrpcMessage {
    type_name: String,
    payload: Vec<u8>,
}

impl GrpcMessage {
    /// Crea un mensaje con nombre de tipo y bytes.
    pub fn new(type_name: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            type_name: type_name.into(),
            payload,
        }
    }

    /// Nombre del tipo lógico.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Carga útil serializada.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Estado gRPC educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrpcStatus {
    code: u16,
    name: &'static str,
    message: String,
}

impl GrpcStatus {
    /// Estado exitoso.
    pub fn ok() -> Self {
        Self::new(0, "OK", "")
    }

    /// Argumento inválido.
    pub fn invalid_argument(message: impl Into<String>) -> Self {
        Self::new(3, "INVALID_ARGUMENT", message)
    }

    /// Recurso no encontrado.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(5, "NOT_FOUND", message)
    }

    /// Servicio no disponible.
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(14, "UNAVAILABLE", message)
    }

    /// Crea un estado explícito.
    pub fn new(code: u16, name: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            name,
            message: message.into(),
        }
    }

    /// Código numérico gRPC.
    pub fn code(&self) -> u16 {
        self.code
    }

    /// Nombre canónico del estado.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Mensaje legible.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Error educativo de gRPC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrpcError {
    DuplicateMethod(String),
    UnknownMethod(String),
    IncompatibleVersion {
        method: String,
        expected: u32,
        actual: u32,
    },
}
