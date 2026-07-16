//! Curso de redes en Rust para Jeresoft Academy.
//!
//! Este crate acompaña el curso `rust-networking`. Cada módulo representa un
//! protocolo o mecanismo canónico del curso y existe para enseñar contratos,
//! invariantes, límites, modos de falla y tradeoffs. Las implementaciones
//! completas se agregan capítulo por capítulo.

pub mod dns;
pub mod grpc;
pub mod http;
pub mod https;
pub mod layers;
pub mod quic;
pub mod smtp;
pub mod tcp;
pub mod tls;
pub mod udp;
pub mod websocket;
