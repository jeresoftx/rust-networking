# Plan de trabajo: rust-networking

Este checklist implementa el curso `rust-networking` como repositorio troncal de
Jeresoft Academy. Se rige por RFC-0001 y por las instrucciones de `AGENTS.md`.

## Principios de ejecución

- [x] Crear repositorio remoto y clonar en `repos/rust-networking`.
- [x] Configurar About de GitHub con descripción clara en español.
- [x] Crear identidad inicial: README, ROADMAP, AGENTS y licencias.
- [x] Crear crate Rust educativo y estructura base.
- [ ] Usar TDD para toda funcionalidad nueva.
- [ ] Hacer commits pequeños y frecuentes con conventional commits.
- [ ] Verificar antes de cada commit cuando aplique:
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all-targets`
  - `cargo test --doc`

## Task 1: Fundación del repositorio

**Files:**
- Create: `README.md`
- Create: `ROADMAP.md`
- Create: `AGENTS.md`
- Create: `LICENSE.md`
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `docs/SUMMARY.md`

- [x] Declarar lugar del curso en Semestre 2.
- [x] Declarar capítulos planeados.
- [x] Declarar límites con `rust-operating-systems`, `rust-async`,
  `rust-crypto` y `rust-distributed-systems`.
- [x] Crear módulos placeholder para que el crate compile desde el primer día.

## Task 2: Modelo de capas, IP y enrutamiento

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

- [ ] Enseñar capas, encapsulación, direcciones IP, subredes, TTL, rutas y
  entrega de mejor esfuerzo.
- [ ] Comparar modelo OSI, TCP/IP y la forma pragmática en que se depuran redes.
- [ ] Incluir tests de clasificación de capas, cálculo de subred simple y rutas.
- [ ] Incluir benchmark cuando exista una operación medible del modelo.

## Task 3: TCP

- [ ] Enseñar conexión, establecimiento en tres pasos, flujo confiable, orden, ventanas,
  retransmisión y cierre.
- [ ] Modelar estados principales sin pretender reemplazar una pila TCP real.
- [ ] Incluir pruebas de transición de estados y segmentos fuera de orden.

## Task 4: UDP

- [ ] Enseñar datagramas, ausencia de conexión, pérdida, duplicación y tamaño.
- [ ] Comparar contra TCP con casos donde la simplicidad gana.
- [ ] Incluir pruebas de entrega de mejor esfuerzo y validación de carga útil.

## Task 5: DNS

- [ ] Enseñar resolución recursiva, autoridad, caché, TTL, registros A, AAAA,
  CNAME, MX y TXT.
- [ ] Incluir modelo de zona y resolver educativo.
- [ ] Incluir pruebas de TTL, alias y registros inexistentes.

## Task 6: TLS

- [ ] Enseñar objetivos de seguridad: confidencialidad, integridad,
  autenticación y negociación.
- [ ] Explicar certificados sin implementar criptografía casera de producción.
- [ ] Incluir modelo de negociación y verificación de identidad.

## Task 7: HTTP

- [ ] Enseñar solicitud/respuesta, métodos, encabezados, códigos de estado,
  cuerpo, conexiones persistentes y semántica de caché.
- [ ] Incluir parser educativo limitado y explícito.
- [ ] Incluir pruebas de solicitudes válidas, errores y límites.

## Task 8: HTTPS

- [ ] Enseñar HTTP sobre TLS, autoridad, certificados, HSTS y errores comunes.
- [ ] Evitar duplicar TLS y HTTP: este capítulo compone los dos contratos.

## Task 9: SMTP

- [ ] Enseñar sesiones, sobre del mensaje, encabezados, MX y límites del correo
  electrónico.
- [ ] Incluir modelo de conversación cliente-servidor.

## Task 10: WebSocket

- [ ] Enseñar actualización desde HTTP, tramas, mensajes, ping/pong y cierre.
- [ ] Comparar contra polling y Server-Sent Events.

## Task 11: gRPC

- [ ] Enseñar contratos, HTTP/2, flujos, códigos de estado y compatibilidad.
- [ ] Comparar contra REST sin convertirlo en guerra de estilos.

## Task 12: QUIC

- [ ] Enseñar transporte sobre UDP, flujos, negociación integrada, migración de
  conexión y relación con HTTP/3.
- [ ] Comparar contra TCP + TLS con foco en latencia y evolución.

## Integración con otros cursos

- [ ] Citar `rust-operating-systems` cuando un mecanismo dependa del kernel.
- [ ] Citar `rust-async` cuando la concurrencia del servidor sea el tema.
- [ ] Citar `rust-crypto` para criptografía interna de TLS.
- [ ] Citar `rust-distributed-systems` para consenso, relojes y comunicación
  entre nodos.
- [ ] Citar `rust-api-design` para diseño de APIs por encima de HTTP/gRPC.

## Finalización del curso

- [ ] Todo ítem público tiene doc-comments con ejemplos.
- [ ] Cada capítulo cumple las doce secciones de RFC-0001 §14.
- [ ] Cada capítulo tiene cuatro a ocho ejercicios en niveles 1 a 4.
- [ ] Cada ejercicio de nivel 1 a 3 tiene solución ejecutable.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.
- [ ] `cargo test --all-targets` pasa.
- [ ] `cargo test --doc` pasa.
- [ ] README y ROADMAP reflejan el estado real.
