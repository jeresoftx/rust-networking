# Rust Networking Course Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Construir todo el contenido del curso `rust-networking` como libro de ingeniería y crate Rust educativo de Jeresoft Academy.

**Architecture:** Cada capítulo vive en un módulo Rust pequeño, un capítulo Markdown, ejemplos progresivos, pruebas, diagrama Mermaid, ejercicios y soluciones. Los modelos son educativos y seguros: primero explican contratos, invariantes y modos de falla; solo después se conectan con APIs reales cuando sea necesario.

**Tech Stack:** Rust 2021, biblioteca estándar, Markdown compatible con mdBook, Mermaid, tests de integración, doctests y benchmarks manuales con `std::time::Instant`.

---

## Reglas De Ejecución

- [x] Crear repositorio remoto y clonar en `repos/rust-networking`.
- [x] Configurar About de GitHub con descripción clara en español.
- [x] Crear identidad inicial: `README.md`, `ROADMAP.md`, `AGENTS.md` y licencias.
- [x] Crear crate Rust educativo y estructura base.
- [ ] Usar TDD para toda funcionalidad nueva: test rojo, implementación mínima, test verde, refactor.
- [ ] Hacer commits pequeños y frecuentes con conventional commits.
- [ ] No agregar dependencias externas sin justificarlo en el capítulo y en el commit.
- [ ] No usar `unsafe` salvo que el capítulo lo requiera y exista comentario `// SAFETY:` más explicación en docs.
- [ ] Mantener español es-MX con acentos, ñ y terminología clara.
- [ ] Verificar antes de cada commit cuando aplique:
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all-targets`
  - `cargo test --doc`
  - `cargo bench --bench <chapter>_bench` cuando el capítulo agregue benchmark
  - `git diff --check`

## Arquitectura Del Repositorio

- [x] `src/lib.rs`: exporta módulos del curso.
- [x] `src/layers.rs`: modelo de capas, IP y enrutamiento.
- [x] `src/tcp.rs`: modelo educativo de TCP.
- [x] `src/udp.rs`: modelo educativo de UDP.
- [x] `src/dns.rs`: modelo educativo de DNS.
- [x] `src/tls.rs`: modelo educativo de TLS.
- [x] `src/http.rs`: modelo educativo de HTTP.
- [x] `src/https.rs`: composición HTTP sobre TLS.
- [x] `src/smtp.rs`: modelo educativo de SMTP.
- [x] `src/websocket.rs`: modelo educativo de WebSocket.
- [x] `src/grpc.rs`: modelo educativo de gRPC.
- [x] `src/quic.rs`: modelo educativo de QUIC.
- [ ] `docs/NN-topic.md`: capítulo con anatomía completa de RFC-0001 §14.
- [ ] `tests/<topic>_test.rs`: pruebas de integración orientadas a comportamiento.
- [ ] `benches/<topic>_bench.rs`: benchmark manual cuando exista operación medible.
- [ ] `diagrams/NN-topic.mmd`: diagrama Mermaid del flujo o protocolo.
- [ ] `examples/<topic>_basic.rs`: ejemplo mínimo.
- [ ] `examples/<topic>_intermediate.rs`: ejemplo con composición o errores.
- [ ] `examples/<topic>_advanced.rs`: ejemplo con tradeoffs explícitos.
- [ ] `examples/<topic>_real_case.rs`: caso realista sin depender de servicios externos.
- [ ] `examples/soluciones/<topic>_*.rs`: soluciones de ejercicios niveles 1 a 3.

## Definición De Terminado Por Capítulo

- [ ] Explica el concepto antes del problema.
- [ ] Explica el problema antes de la implementación.
- [ ] Compara alternativas y justifica el diseño elegido.
- [ ] Declara garantías, límites, invariantes y modos de falla.
- [ ] Incluye sección de historia, fundamentos, casos de uso, ventajas y limitaciones.
- [ ] Incluye análisis de complejidad cuando hay operaciones modeladas.
- [ ] Incluye diagrama Mermaid y referencia desde el capítulo.
- [ ] Incluye implementación con doc-comments y ejemplos compilables.
- [ ] Incluye tests de integración y doctests.
- [ ] Incluye benchmark o declara por qué no aplica.
- [ ] Incluye cuatro a ocho ejercicios entre niveles 1 y 4.
- [ ] Incluye soluciones ejecutables para niveles 1 a 3.
- [ ] Actualiza `README.md`, `ROADMAP.md` y este checklist.
- [ ] Corre verificaciones completas antes del commit.
- [ ] Hace commit pequeño y empuja a `origin/main` cuando el estado está verde.

## Task 1: Fundación Del Repositorio

**Files:**
- Created: `README.md`
- Created: `ROADMAP.md`
- Created: `AGENTS.md`
- Created: `LICENSE.md`
- Created: `Cargo.toml`
- Created: `src/lib.rs`
- Created: `docs/SUMMARY.md`

- [x] Declarar lugar del curso en Semestre 2.
- [x] Declarar capítulos planeados.
- [x] Declarar límites con `rust-operating-systems`, `rust-async`, `rust-crypto` y `rust-distributed-systems`.
- [x] Crear módulos placeholder para que el crate compile desde el primer día.
- [x] Ejecutar `cargo fmt --check`.
- [x] Ejecutar `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] Ejecutar `cargo test --all-targets`.
- [x] Ejecutar `cargo test --doc`.
- [x] Commit: `chore: scaffold rust networking course`.

## Task 2: Modelo De Capas, IP Y Enrutamiento

**Files:**
- Create: `docs/01-layers-ip-routing.md`
- Modify: `src/layers.rs`
- Create: `tests/layers_test.rs`
- Create: `benches/layers_bench.rs`
- Create: `diagrams/01-layers-ip-routing.mmd`
- Create: `examples/layers_basic.rs`
- Create: `examples/layers_intermediate.rs`
- Create: `examples/layers_advanced.rs`
- Create: `examples/layers_real_case.rs`
- Create: `examples/soluciones/layers_classify_packet.rs`
- Create: `examples/soluciones/layers_subnet_match.rs`
- Create: `examples/soluciones/layers_route_selection.rs`

- [x] Diseñar API mínima: `NetworkLayer`, `EncapsulatedFrame`, `Ipv4Address`, `Ipv4Cidr`, `Route`, `RoutingTable`, `RouteDecision`.
- [x] Escribir test rojo para clasificar capas y encapsulación.
- [x] Implementar clasificación de capas y encapsulación mínima.
- [x] Escribir test rojo para pertenencia de IPv4 a CIDR.
- [x] Implementar `Ipv4Cidr::contains`.
- [x] Escribir test rojo para selección de ruta por prefijo más específico.
- [x] Implementar `RoutingTable::select_route`.
- [x] Escribir test rojo para TTL agotado.
- [x] Implementar decremento de TTL y error educativo.
- [x] Documentar capas, encapsulación, IP, subredes, TTL, rutas y entrega de mejor esfuerzo.
- [x] Comparar modelo OSI, TCP/IP y depuración pragmática.
- [x] Crear diagrama de encapsulación y selección de ruta.
- [x] Crear ejemplos básico, intermedio, avanzado y caso real.
- [x] Crear ejercicios sobre capas, CIDR, rutas y diagnóstico.
- [x] Crear soluciones ejecutables niveles 1 a 3.
- [x] Crear benchmark de selección de ruta con tablas pequeñas y medianas.
- [x] Actualizar estado a `benchmarked` en README y ROADMAP.
- [x] Verificar y hacer commit: `feat: add layers ip routing chapter`.

## Task 3: TCP

**Files:**
- Create: `docs/02-tcp.md`
- Modify: `src/tcp.rs`
- Create: `tests/tcp_test.rs`
- Create: `benches/tcp_bench.rs`
- Create: `diagrams/02-tcp.mmd`
- Create: `examples/tcp_basic.rs`
- Create: `examples/tcp_intermediate.rs`
- Create: `examples/tcp_advanced.rs`
- Create: `examples/tcp_real_case.rs`
- Create: `examples/soluciones/tcp_handshake.rs`
- Create: `examples/soluciones/tcp_ordered_stream.rs`
- Create: `examples/soluciones/tcp_retransmission.rs`

- [x] Diseñar API mínima: `TcpState`, `TcpSegment`, `SequenceNumber`, `TcpConnection`, `TcpEvent`, `TcpError`.
- [x] Escribir test rojo para establecimiento en tres pasos.
- [x] Implementar transiciones `Closed -> SynSent -> Established`.
- [x] Escribir test rojo para rechazo de segmento inesperado.
- [x] Implementar errores de transición.
- [x] Escribir test rojo para ordenar segmentos por número de secuencia.
- [x] Implementar buffer educativo de reordenamiento.
- [x] Escribir test rojo para retransmisión por acuse faltante.
- [x] Implementar modelo mínimo de retransmisión.
- [x] Escribir test rojo para cierre con `FIN`.
- [x] Implementar cierre ordenado.
- [x] Documentar conexión, confiabilidad, orden, ventanas, retransmisión y cierre.
- [x] Comparar TCP contra UDP y QUIC.
- [x] Crear diagrama de estados.
- [x] Crear ejemplos progresivos y caso real de solicitud sobre flujo confiable.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de reordenamiento de segmentos.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add tcp chapter`.

## Task 4: UDP

**Files:**
- Create: `docs/03-udp.md`
- Modify: `src/udp.rs`
- Create: `tests/udp_test.rs`
- Create: `benches/udp_bench.rs`
- Create: `diagrams/03-udp.mmd`
- Create: `examples/udp_basic.rs`
- Create: `examples/udp_intermediate.rs`
- Create: `examples/udp_advanced.rs`
- Create: `examples/udp_real_case.rs`
- Create: `examples/soluciones/udp_datagram.rs`
- Create: `examples/soluciones/udp_loss_model.rs`
- Create: `examples/soluciones/udp_size_limit.rs`

- [x] Diseñar API mínima: `UdpDatagram`, `UdpEndpoint`, `DeliveryOutcome`, `UdpError`.
- [x] Escribir test rojo para datagrama con origen, destino y carga útil.
- [x] Implementar constructor y validación de tamaño.
- [x] Escribir test rojo para entrega de mejor esfuerzo con pérdida simulada determinista.
- [x] Implementar modelo determinista de pérdida, duplicación y entrega.
- [x] Escribir test rojo para carga útil demasiado grande.
- [x] Implementar error de tamaño.
- [x] Documentar datagramas, ausencia de conexión, pérdida, duplicación y tamaño.
- [x] Comparar UDP contra TCP y QUIC.
- [x] Crear diagrama de datagramas independientes.
- [x] Crear ejemplos progresivos y caso real de telemetría tolerante a pérdida.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de validación y despacho de datagramas.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add udp chapter`.

## Task 5: DNS

**Files:**
- Create: `docs/04-dns.md`
- Modify: `src/dns.rs`
- Create: `tests/dns_test.rs`
- Create: `benches/dns_bench.rs`
- Create: `diagrams/04-dns.mmd`
- Create: `examples/dns_basic.rs`
- Create: `examples/dns_intermediate.rs`
- Create: `examples/dns_advanced.rs`
- Create: `examples/dns_real_case.rs`
- Create: `examples/soluciones/dns_a_record.rs`
- Create: `examples/soluciones/dns_cname_resolution.rs`
- Create: `examples/soluciones/dns_ttl_cache.rs`

- [x] Diseñar API mínima: `DomainName`, `DnsRecord`, `RecordType`, `Zone`, `Resolver`, `Resolution`, `DnsError`.
- [x] Escribir test rojo para registro A.
- [x] Implementar zona con registros A y AAAA.
- [x] Escribir test rojo para CNAME encadenado con límite de saltos.
- [x] Implementar resolución de alias.
- [x] Escribir test rojo para TTL y caché expirada.
- [x] Implementar caché educativa con reloj inyectado como valor.
- [x] Escribir test rojo para NXDOMAIN.
- [x] Implementar error de nombre inexistente.
- [x] Documentar resolución recursiva, autoridad, caché, TTL, A, AAAA, CNAME, MX y TXT.
- [x] Comparar DNS contra archivos hosts y descubrimiento de servicios.
- [x] Crear diagrama de resolución.
- [x] Crear ejemplos progresivos y caso real de resolución de API.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de resolución con caché fría y caliente.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add dns chapter`.

## Task 6: TLS

**Files:**
- Create: `docs/05-tls.md`
- Modify: `src/tls.rs`
- Create: `tests/tls_test.rs`
- Create: `benches/tls_bench.rs`
- Create: `diagrams/05-tls.mmd`
- Create: `examples/tls_basic.rs`
- Create: `examples/tls_intermediate.rs`
- Create: `examples/tls_advanced.rs`
- Create: `examples/tls_real_case.rs`
- Create: `examples/soluciones/tls_identity_check.rs`
- Create: `examples/soluciones/tls_cipher_negotiation.rs`
- Create: `examples/soluciones/tls_certificate_chain.rs`

- [x] Diseñar API mínima: `Certificate`, `CertificateChain`, `TlsVersion`, `CipherSuite`, `TlsClientHello`, `TlsServerHello`, `TlsHandshake`, `TlsError`.
- [x] Escribir test rojo para coincidencia de nombre del servidor.
- [x] Implementar verificación educativa de identidad.
- [x] Escribir test rojo para cadena de certificados incompleta.
- [x] Implementar validación estructural de cadena sin criptografía real.
- [x] Escribir test rojo para negociación de versión y cipher suite.
- [x] Implementar selección por intersección de capacidades.
- [x] Escribir test rojo para rechazo de algoritmo obsoleto.
- [x] Implementar política mínima de rechazo.
- [x] Documentar confidencialidad, integridad, autenticación, negociación y certificados.
- [x] Declarar explícitamente que no se implementa criptografía de producción.
- [x] Comparar TLS contra cifrado casero, VPN y texto plano.
- [x] Crear diagrama de negociación.
- [x] Crear ejemplos progresivos y caso real de identidad de API.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de negociación estructural.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add tls chapter`.

## Task 7: HTTP

**Files:**
- Create: `docs/06-http.md`
- Modify: `src/http.rs`
- Create: `tests/http_test.rs`
- Create: `benches/http_bench.rs`
- Create: `diagrams/06-http.mmd`
- Create: `examples/http_basic.rs`
- Create: `examples/http_intermediate.rs`
- Create: `examples/http_advanced.rs`
- Create: `examples/http_real_case.rs`
- Create: `examples/soluciones/http_parse_request.rs`
- Create: `examples/soluciones/http_status_response.rs`
- Create: `examples/soluciones/http_cache_headers.rs`

- [x] Diseñar API mínima: `HttpMethod`, `HttpVersion`, `HeaderMap`, `HttpRequest`, `HttpResponse`, `StatusCode`, `HttpParseError`.
- [x] Escribir test rojo para parsear solicitud GET simple.
- [x] Implementar parser educativo limitado de línea inicial y encabezados.
- [x] Escribir test rojo para método inválido.
- [x] Implementar error de método.
- [x] Escribir test rojo para respuesta con código de estado y cuerpo.
- [x] Implementar construcción de respuesta.
- [x] Escribir test rojo para `Cache-Control` y `ETag` como metadatos.
- [x] Implementar helpers de caché.
- [x] Documentar solicitud/respuesta, métodos, encabezados, códigos de estado, cuerpo, conexiones persistentes y caché.
- [x] Comparar HTTP/1.1 contra HTTP/2 y gRPC.
- [x] Crear diagrama de solicitud-respuesta.
- [x] Crear ejemplos progresivos y caso real de punto de entrada educativo.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de parseo limitado.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add http chapter`.

## Task 8: HTTPS

**Files:**
- Create: `docs/07-https.md`
- Modify: `src/https.rs`
- Create: `tests/https_test.rs`
- Create: `benches/https_bench.rs`
- Create: `diagrams/07-https.mmd`
- Create: `examples/https_basic.rs`
- Create: `examples/https_intermediate.rs`
- Create: `examples/https_advanced.rs`
- Create: `examples/https_real_case.rs`
- Create: `examples/soluciones/https_secure_request.rs`
- Create: `examples/soluciones/https_certificate_failure.rs`
- Create: `examples/soluciones/https_hsts_policy.rs`

- [x] Diseñar API mínima: `HttpsRequest`, `HttpsPolicy`, `HstsPolicy`, `SecureTransport`, `HttpsError`.
- [x] Escribir test rojo para componer una solicitud HTTP sobre sesión TLS válida.
- [x] Implementar composición educativa con tipos de `http` y `tls`.
- [x] Escribir test rojo para certificado inválido.
- [x] Implementar propagación de error TLS.
- [x] Escribir test rojo para política HSTS.
- [x] Implementar decisión de forzar HTTPS.
- [x] Documentar HTTP sobre TLS, autoridad, certificados, HSTS y errores comunes.
- [x] Evitar reexplicar HTTP y TLS desde cero; citar capítulos 05 y 06.
- [x] Comparar HTTPS contra HTTP plano y TLS mal configurado.
- [x] Crear diagrama de composición.
- [x] Crear ejemplos progresivos y caso real de cliente que rechaza identidad incorrecta.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark solo si la composición tiene operación medible; si no, documentar que no aplica.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add https chapter`.

## Task 9: SMTP

**Files:**
- Create: `docs/08-smtp.md`
- Modify: `src/smtp.rs`
- Create: `tests/smtp_test.rs`
- Create: `benches/smtp_bench.rs`
- Create: `diagrams/08-smtp.mmd`
- Create: `examples/smtp_basic.rs`
- Create: `examples/smtp_intermediate.rs`
- Create: `examples/smtp_advanced.rs`
- Create: `examples/smtp_real_case.rs`
- Create: `examples/soluciones/smtp_conversation.rs`
- Create: `examples/soluciones/smtp_envelope_headers.rs`
- Create: `examples/soluciones/smtp_mx_selection.rs`

- [x] Diseñar API mínima: `SmtpCommand`, `SmtpReply`, `MailEnvelope`, `EmailHeaders`, `SmtpSession`, `SmtpError`.
- [x] Escribir test rojo para conversación `HELO`, `MAIL FROM`, `RCPT TO`, `DATA`.
- [x] Implementar máquina de sesión educativa.
- [x] Escribir test rojo para separar sobre del mensaje y encabezados.
- [x] Implementar `MailEnvelope` y `EmailHeaders`.
- [x] Escribir test rojo para selección de MX por prioridad.
- [x] Implementar ordenamiento de MX.
- [x] Escribir test rojo para comando fuera de orden.
- [x] Implementar error de secuencia.
- [x] Documentar sesiones, sobre del mensaje, encabezados, MX y límites del correo electrónico.
- [x] Comparar SMTP contra HTTP APIs de envío de correo.
- [x] Crear diagrama de sesión.
- [x] Crear ejemplos progresivos y caso real de cola de correo saliente.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de validación de comandos.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add smtp chapter`.

## Task 10: WebSocket

**Files:**
- Create: `docs/09-websocket.md`
- Modify: `src/websocket.rs`
- Create: `tests/websocket_test.rs`
- Create: `benches/websocket_bench.rs`
- Create: `diagrams/09-websocket.mmd`
- Create: `examples/websocket_basic.rs`
- Create: `examples/websocket_intermediate.rs`
- Create: `examples/websocket_advanced.rs`
- Create: `examples/websocket_real_case.rs`
- Create: `examples/soluciones/websocket_upgrade.rs`
- Create: `examples/soluciones/websocket_frames.rs`
- Create: `examples/soluciones/websocket_ping_pong.rs`

- [x] Diseñar API mínima: `WebSocketUpgrade`, `WebSocketFrame`, `Opcode`, `CloseCode`, `WebSocketState`, `WebSocketError`.
- [x] Escribir test rojo para actualización desde HTTP.
- [x] Implementar validación educativa de actualización.
- [x] Escribir test rojo para frame de texto.
- [x] Implementar frame con opcode y carga útil.
- [x] Escribir test rojo para ping/pong.
- [x] Implementar respuesta pong.
- [x] Escribir test rojo para cierre ordenado.
- [x] Implementar transición a cerrado.
- [x] Documentar actualización desde HTTP, tramas, mensajes, ping/pong y cierre.
- [x] Comparar contra polling y Server-Sent Events.
- [x] Crear diagrama de actualización y tramas.
- [x] Crear ejemplos progresivos y caso real de notificaciones.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de codificación/decodificación de tramas educativas.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add websocket chapter`.

## Task 11: gRPC

**Files:**
- Create: `docs/10-grpc.md`
- Modify: `src/grpc.rs`
- Create: `tests/grpc_test.rs`
- Create: `benches/grpc_bench.rs`
- Create: `diagrams/10-grpc.mmd`
- Create: `examples/grpc_basic.rs`
- Create: `examples/grpc_intermediate.rs`
- Create: `examples/grpc_advanced.rs`
- Create: `examples/grpc_real_case.rs`
- Create: `examples/soluciones/grpc_method_contract.rs`
- Create: `examples/soluciones/grpc_status_mapping.rs`
- Create: `examples/soluciones/grpc_streaming_model.rs`

- [x] Diseñar API mínima: `GrpcMethod`, `GrpcService`, `GrpcMessage`, `GrpcStatus`, `StreamMode`, `GrpcError`.
- [x] Escribir test rojo para contrato de servicio y método.
- [x] Implementar registro educativo de métodos.
- [x] Escribir test rojo para mapeo de códigos de estado.
- [x] Implementar `GrpcStatus`.
- [x] Escribir test rojo para modo unario, flujo del servidor, flujo del cliente y bidireccional.
- [x] Implementar `StreamMode`.
- [x] Escribir test rojo para compatibilidad de versión de contrato.
- [x] Implementar verificación simple de versión.
- [x] Documentar contratos, HTTP/2, flujos, códigos de estado y compatibilidad.
- [x] Comparar contra REST sin convertirlo en guerra de estilos.
- [x] Crear diagrama de llamada y stream.
- [x] Crear ejemplos progresivos y caso real de servicio interno.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de validación de contratos.
- [x] Actualizar estado y checklist.
- [x] Verificar y hacer commit: `feat: add grpc chapter`.

## Task 12: QUIC

**Files:**
- Create: `docs/11-quic.md`
- Modify: `src/quic.rs`
- Create: `tests/quic_test.rs`
- Create: `benches/quic_bench.rs`
- Create: `diagrams/11-quic.mmd`
- Create: `examples/quic_basic.rs`
- Create: `examples/quic_intermediate.rs`
- Create: `examples/quic_advanced.rs`
- Create: `examples/quic_real_case.rs`
- Create: `examples/soluciones/quic_streams.rs`
- Create: `examples/soluciones/quic_connection_migration.rs`
- Create: `examples/soluciones/quic_handshake.rs`

- [x] Diseñar API mínima: `QuicConnectionId`, `QuicStreamId`, `QuicPacket`, `QuicStream`, `ConnectionMigration`, `QuicError`.
- [x] Escribir test rojo para múltiples flujos independientes.
- [x] Implementar modelo de flujos sin bloqueo entre ellos.
- [x] Escribir test rojo para migración de conexión por cambio de dirección.
- [x] Implementar `ConnectionMigration`.
- [x] Escribir test rojo para negociación integrada con seguridad.
- [x] Implementar modelo educativo de negociación.
- [x] Escribir test rojo para pérdida de paquete en un flujo sin detener otro.
- [x] Implementar aislamiento de flujos.
- [ ] Documentar transporte sobre UDP, flujos, negociación integrada, migración de conexión y HTTP/3.
- [ ] Comparar contra TCP + TLS con foco en latencia y evolución.
- [ ] Crear diagrama de conexión, paquetes y flujos.
- [ ] Crear ejemplos progresivos y caso real de conexión móvil.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de despacho de paquetes por flujo.
- [ ] Actualizar estado y checklist.
- [ ] Verificar y hacer commit: `feat: add quic chapter`.

## Task 13: Integración Entre Cursos

**Files:**
- Modify: `README.md`
- Modify: `ROADMAP.md`
- Modify: `docs/01-layers-ip-routing.md`
- Modify: `docs/02-tcp.md`
- Modify: `docs/03-udp.md`
- Modify: `docs/04-dns.md`
- Modify: `docs/05-tls.md`
- Modify: `docs/06-http.md`
- Modify: `docs/07-https.md`
- Modify: `docs/09-websocket.md`
- Modify: `docs/10-grpc.md`
- Modify: `docs/11-quic.md`

- [ ] Citar `rust-operating-systems` cuando un mecanismo dependa del kernel.
- [ ] Citar `rust-async` cuando la concurrencia del servidor sea el tema.
- [ ] Citar `rust-crypto` para criptografía interna de TLS.
- [ ] Citar `rust-distributed-systems` para consenso, relojes y comunicación entre nodos.
- [ ] Citar `rust-api-design` para diseño de APIs por encima de HTTP/gRPC.
- [ ] Citar `rust-cloud` para VPC, balanceo y servicios administrados.
- [ ] Citar `rust-devops` para observabilidad, diagnóstico y operación.
- [ ] Citar `rust-travel` donde redes sostenga búsquedas, reservas e integraciones externas.
- [ ] Mantener cada referencia como nota de camino, no como reexplicación.
- [ ] Verificar y hacer commit: `docs: add networking cross-course links`.

## Task 14: Revisión Editorial Y Ortográfica

**Files:**
- Modify: `docs/*.md`
- Modify: `README.md`
- Modify: `ROADMAP.md`
- Modify: `AGENTS.md`

- [ ] Buscar errores comunes: `canonico`, `prerequisitos`, `reclamacion`, `epoca`, `critica`, `seccion`, `retencion`, `senal`, `contrasena`.
- [ ] Buscar anglicismos evitables: `request`, `response`, `headers`, `payload`, `best-effort`, `handshake`, `streaming`, `frames`, `upgrade`.
- [ ] Mantener términos técnicos cuando sean nombres de protocolo, API o convención aceptada: `HTTP`, `TLS`, `WebSocket`, `gRPC`, `QUIC`, `ETag`.
- [ ] Revisar que cada capítulo use español es-MX natural y consistente.
- [ ] Revisar que no haya capítulos marcados como completos si están parciales.
- [ ] Verificar y hacer commit: `docs: polish networking course language`.

## Task 15: Finalización Del Curso

**Files:**
- Modify: `README.md`
- Modify: `ROADMAP.md`
- Modify: `docs/SUMMARY.md`
- Modify: `docs/superpowers/plans/2026-07-16-rust-networking-course.md`

- [ ] Confirmar que todo ítem público tiene doc-comments con ejemplos.
- [ ] Confirmar que cada capítulo cumple las doce secciones de RFC-0001 §14.
- [ ] Confirmar que cada capítulo tiene cuatro a ocho ejercicios.
- [ ] Confirmar que cada ejercicio de nivel 1 a 3 tiene solución ejecutable.
- [ ] Confirmar que cada benchmark agregado corre.
- [ ] Ejecutar `cargo fmt --check`.
- [ ] Ejecutar `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Ejecutar `cargo test --all-targets`.
- [ ] Ejecutar `cargo test --doc`.
- [ ] Ejecutar `cargo bench`.
- [ ] Ejecutar `git diff --check`.
- [ ] Confirmar que README y ROADMAP reflejan el estado real.
- [ ] Confirmar que los topics y About de GitHub reflejan la identidad del curso.
- [ ] Hacer commit final: `docs: complete networking course checklist`.
