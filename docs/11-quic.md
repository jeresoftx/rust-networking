# QUIC

> **Curso:** rust-networking · **Capítulo:** 11 · **Prerrequisitos:** UDP,
> TLS, HTTP/2 y gRPC conceptual
> **Código:** [`src/quic.rs`](../src/quic.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

QUIC es un protocolo de transporte moderno construido sobre UDP. Su objetivo no
es volver a inventar TCP línea por línea, sino resolver varias fricciones que
aparecen al operar conexiones seguras en redes reales: negociación lenta,
bloqueo entre flujos, migración de conexión y evolución del protocolo.

Este capítulo no implementa red real ni criptografía. Modela las ideas que un
ingeniero debe entender antes de usar HTTP/3, bibliotecas QUIC o plataformas
que prometen menor latencia por arte de magia.

## Motivación

En una red móvil, la dirección visible de un cliente puede cambiar al pasar de
Wi-Fi a datos celulares. En TCP tradicional, la conexión está fuertemente
atada al par dirección-puerto. Si ese par cambia, la sesión suele romperse y la
aplicación debe reconstruir estado.

Además, cuando varias respuestas comparten una conexión, una pérdida de paquete
puede frenar más trabajo del necesario. La idea central de QUIC es:

```text
QUIC conserva una identidad de conexión y aísla flujos encima de UDP con
seguridad integrada.
```

## Teoría

### Historia

QUIC nació para reducir latencia y facilitar evolución del transporte en la web
moderna. Se estandarizó como base de HTTP/3 y colocó muchas decisiones que antes
vivían entre TCP, TLS y HTTP/2 en un diseño coordinado.

### Fundamentos

El modelo del crate usa:

- `QuicConnectionId`: identidad lógica de conexión;
- `QuicStreamId`: identificador de flujo independiente;
- `QuicPacket`: paquete asociado a conexión, flujo, secuencia y carga útil;
- `QuicStream`: estado de recepción por flujo;
- `ConnectionMigration`: cambio de ruta que conserva identidad;
- `QuicHandshake`: negociación educativa de TLS 1.3 y `h3`;
- `QuicError`: errores de contrato del modelo.

### Transporte sobre UDP

QUIC usa UDP como sustrato porque UDP permite que el protocolo evolucione en
espacio de usuario. Eso no significa que QUIC sea "UDP con esteroides" sin
estructura: QUIC agrega conexión, flujos, retransmisión, control de congestión,
seguridad y negociación.

En este curso separamos ambas ideas:

- UDP entrega datagramas sin conexión.
- QUIC construye una sesión segura con flujos encima de datagramas.

### Flujos independientes

Una conexión QUIC puede transportar varios flujos. Cada flujo tiene su propia
secuencia lógica. Si un paquete se pierde en un flujo, ese flujo puede esperar
retransmisión, pero otro flujo no necesita quedar bloqueado por la misma pérdida.

Este punto es clave para HTTP/3: varias solicitudes o respuestas pueden convivir
sin heredar todos los problemas de bloqueo de una sola secuencia compartida.

### Negociación integrada

QUIC integra TLS 1.3 como parte del transporte. En la práctica, no se piensa
como "primero conecto y después agrego seguridad" de la misma forma que muchos
equipos aprendieron con TCP + TLS. La seguridad está dentro del diseño del
protocolo.

El modelo educativo acepta `TLS 1.3` y `h3` para subrayar dos ideas:

- la versión de seguridad importa;
- el protocolo de aplicación negociado también es parte del contrato.

### Migración de conexión

La migración ocurre cuando cambia la ruta visible, pero se conserva la identidad
de conexión. Un cliente móvil puede moverse de una red a otra sin obligar a la
aplicación a reconstruir toda la sesión lógica.

`ConnectionMigration` no valida redes reales. Registra la ruta anterior, la ruta
actual, el motivo y el identificador preservado.

### HTTP/3

HTTP/3 usa QUIC como transporte. Eso no convierte a QUIC en "HTTP"; QUIC es la
capa que proporciona flujos, seguridad, conexión y evolución. HTTP/3 coloca su
semántica de solicitudes y respuestas encima.

### Comparación con TCP + TLS

TCP + TLS sigue siendo fundamental, probado y ampliamente disponible. QUIC no lo
borra. La comparación sana se centra en tradeoffs:

- TCP + TLS tiene madurez, herramientas y compatibilidad enormes.
- QUIC puede reducir rondas de negociación y manejar mejor cambios de red.
- TCP expone menos flexibilidad a nivel de aplicación.
- QUIC mueve más responsabilidad al espacio de usuario y requiere observabilidad
  distinta.

Elegir QUIC no debe ser una moda. Debe responder a latencia, movilidad,
multiplexación, compatibilidad operativa y soporte de infraestructura.

## Diagramas

El diagrama principal vive en
[`diagrams/11-quic.mmd`](../diagrams/11-quic.mmd). Muestra conexión, flujos,
paquetes, negociación y migración de ruta.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `QuicConnectionId::new` | O(c) | O(c) | O(c) | O(c) |
| `QuicPacket::new` | O(1) | O(1) | O(1) | O(p) |
| `QuicStream::receive` | O(log n) | O(log n) | O(log n) | O(p) |
| `payloads_in_order` | O(n) | O(n) | O(n) | O(n) |
| `QuicHandshake::negotiate` | O(t + a) | O(t + a) | O(t + a) | O(t + a) |

`c` es la longitud del identificador de conexión, `p` el tamaño de la carga
útil, `n` la cantidad de paquetes recibidos, `t` la versión TLS y `a` el
protocolo de aplicación.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar tres flujos, pérdida
en uno de ellos y migración de Wi-Fi a datos celulares.

## Implementación

La implementación vive en [`src/quic.rs`](../src/quic.rs). El modelo usa mapas
ordenados para conservar cargas útiles por secuencia dentro de cada flujo. La
pérdida se registra por flujo, no por conexión global, para enseñar el
aislamiento que diferencia a QUIC de un flujo único compartido.

## Pruebas

Las pruebas cubren:

- recepción de paquetes fuera de orden dentro de flujos independientes;
- migración de conexión con identidad preservada;
- negociación integrada con TLS 1.3 y `h3`;
- rechazo de negociación con TLS obsoleto;
- pérdida de paquete aislada por flujo;
- rechazo de paquetes dirigidos al flujo incorrecto.

## Benchmarks

El benchmark manual vive en
[`benches/quic_bench.rs`](../benches/quic_bench.rs). Mide despacho de paquetes
por flujo, negociación educativa y registro de migraciones.

## Ejercicios

### Ejercicio 1: Flujos independientes `[Nivel 1]`

Crea dos flujos QUIC y recibe un paquete en cada uno.

**Entrada/Salida esperada:** cada flujo debe conservar solo sus propias cargas
útiles.

### Ejercicio 2: Migración de conexión `[Nivel 2]`

Registra una migración de `wifi` a `lte`.

**Entrada/Salida esperada:** el identificador de conexión debe ser el mismo
antes y después del cambio.

### Ejercicio 3: Negociación segura `[Nivel 3]`

Negocia una conexión con `TLS 1.3` y `h3`.

**Entrada/Salida esperada:** `is_secure` debe devolver `true`.

### Ejercicio 4: ¿QUIC o TCP + TLS? `[Nivel 4]`

Decide qué usarías para una aplicación de videollamadas móviles. Justifica con
latencia, cambios de red, observabilidad y compatibilidad.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/quic_streams.rs`](../examples/soluciones/quic_streams.rs)
- [`examples/soluciones/quic_connection_migration.rs`](../examples/soluciones/quic_connection_migration.rs)
- [`examples/soluciones/quic_handshake.rs`](../examples/soluciones/quic_handshake.rs)
