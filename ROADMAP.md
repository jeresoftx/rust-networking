# ROADMAP

Estado de avance de `rust-networking`, repositorio del camino troncal de
Jeresoft Academy para redes en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado Actual

El repositorio ya tiene la fundación del curso y los dos primeros capítulos
desarrollados: modelo de capas/IP/enrutamiento y TCP. La siguiente línea natural
es continuar con UDP sin perder la anatomía completa de RFC-0001 §14.

El checklist detallado vive en
[`docs/superpowers/plans/2026-07-16-rust-networking-course.md`](docs/superpowers/plans/2026-07-16-rust-networking-course.md).

## Capítulos Planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Modelo de capas, IP y enrutamiento | benchmarked |
| 02 | TCP | benchmarked |
| 03 | UDP | planned |
| 04 | DNS | planned |
| 05 | TLS | planned |
| 06 | HTTP | planned |
| 07 | HTTPS | planned |
| 08 | SMTP | planned |
| 09 | WebSocket | planned |
| 10 | gRPC | planned |
| 11 | QUIC | planned |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.

## Fuera De Alcance Por Ahora

- Programación asíncrona con Tokio: vive en `rust-async`, salvo comparaciones
  necesarias.
- Internals profundos de sistemas operativos: viven en `rust-operating-systems`.
- Criptografía de producción: vive en `rust-crypto`; aquí TLS se estudia como
  protocolo y contrato de seguridad.
- Sistemas distribuidos: viven en `rust-distributed-systems`, salvo notas de
  camino.
- Implementaciones `unsafe` avanzadas sin justificación escrita y revisión
  humana explícita.
