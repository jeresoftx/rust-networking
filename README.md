# Rust Networking

Repositorio del camino troncal de Jeresoft Academy para estudiar redes en Rust.
Pertenece al Semestre 2 del plan de estudios junto con
`rust-operating-systems` (RFC-0001 §10).

El objetivo no es solo mostrar APIs de sockets. El objetivo es crear un recurso
educativo completo: cada protocolo debe explicar por qué existe, qué problema
resuelve, qué garantías ofrece, qué no garantiza, cómo se modela, cómo se
prueba y cómo se mide.

## Qué Contiene

- Capítulos en Markdown compatibles con mdBook.
- Modelos Rust idiomáticos, un protocolo o mecanismo por módulo.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks que confrontan el análisis teórico con mediciones.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar En El Camino

Este curso vive en el Semestre 2. Recibe ideas de estructuras de datos,
algoritmos y Rust básico, y alimenta sistemas operativos, bases de datos
internals, sistemas distribuidos, cloud, DevOps, API design, travel tech y
mensajería.

Redes es canónico aquí: modelo de capas, IP, enrutamiento, TCP, UDP, DNS, TLS,
HTTP, HTTPS, SMTP, WebSocket, gRPC y QUIC se explican en este repositorio antes
de reutilizarse en cursos posteriores.

## Capítulos

| # | Capítulo | Módulo | Estado |
|---|----------|--------|--------|
| 01 | Modelo de capas, IP y enrutamiento | `src/layers.rs` | benchmarked |
| 02 | TCP | `src/tcp.rs` | planned |
| 03 | UDP | `src/udp.rs` | planned |
| 04 | DNS | `src/dns.rs` | planned |
| 05 | TLS | `src/tls.rs` | planned |
| 06 | HTTP | `src/http.rs` | planned |
| 07 | HTTPS | `src/https.rs` | planned |
| 08 | SMTP | `src/smtp.rs` | planned |
| 09 | WebSocket | `src/websocket.rs` | planned |
| 10 | gRPC | `src/grpc.rs` | planned |
| 11 | QUIC | `src/quic.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

## Estructura Esperada

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
docs/
  SUMMARY.md
src/
  lib.rs
examples/
  soluciones/
tests/
benches/
diagrams/
assets/
```

## Cómo Usarlo

Ejecutar tests:

```bash
cargo test
```

Formatear:

```bash
cargo fmt
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

Ejecutar benchmarks:

```bash
cargo bench
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha límite.
- `docs/superpowers/plans/2026-07-16-rust-networking-course.md` contiene el
  checklist de implementación inicial.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
