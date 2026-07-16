# UDP

> **Curso:** rust-networking · **Capítulo:** 03 · **Prerrequisitos:** modelo de
> capas, IP, enrutamiento y diferencias básicas entre TCP y transporte sin
> conexión
> **Código:** [`src/udp.rs`](../src/udp.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

UDP existe para enviar datagramas independientes entre procesos sin establecer
una conexión previa. No promete orden, entrega, retransmisión ni ausencia de
duplicados. Esa ausencia de garantías no es una carencia accidental: es parte de
su contrato.

Este capítulo enseña cuándo una abstracción mínima es suficiente. UDP deja a la
aplicación decidir si necesita tolerar pérdida, repetir mensajes, descartar
duplicados, numerar eventos o aceptar que algunos datos nunca llegarán.

## Motivación

No todo mensaje merece una conexión confiable. En telemetría, voz, video,
descubrimiento local o señales periódicas, un dato viejo puede valer menos que
un dato nuevo. Si una lectura de temperatura se pierde, quizá la siguiente
lectura basta. Retransmitir todo puede aumentar latencia y empeorar el sistema.

La idea central es:

```text
UDP entrega datagramas de mejor esfuerzo; la política vive en la aplicación.
```

## Teoría

### Historia

UDP apareció como una alternativa simple a TCP para aplicaciones que no
necesitan una conversación confiable y ordenada. Conserva puertos, origen,
destino y una carga útil, pero evita establecimiento, estado de conexión,
acuse, reordenamiento y retransmisión.

### Fundamentos

Un datagrama UDP es una unidad independiente. Cada envío contiene origen,
destino y carga útil. Si dos datagramas pertenecen a una misma secuencia lógica,
UDP no lo sabe; esa relación debe modelarla la aplicación.

En este crate, `UdpDatagram` valida el tamaño máximo práctico de carga útil para
UDP sobre IPv4 sin opciones: 65,507 bytes. Ese límite sale de restar encabezado
IPv4 y encabezado UDP al tamaño máximo de un paquete IPv4.

`DeliveryOutcome` modela entrega de mejor esfuerzo de forma determinista para
pruebas:

```text
múltiplos de 5 -> pérdida
múltiplos de 3 -> duplicación
otros valores  -> entrega única
```

Si un número cumple pérdida y duplicación, gana pérdida. Esta regla no pretende
imitar una red real; existe para enseñar que la aplicación debe tratar esos tres
casos como posibles.

### Casos de uso

UDP suele aparecer en:

- telemetría tolerante a pérdida;
- voz y video en tiempo real;
- descubrimiento local;
- DNS tradicional;
- protocolos propios donde la aplicación implementa su propia confiabilidad;
- QUIC, que construye un transporte moderno encima de UDP.

### Ventajas y limitaciones

Ventajas:

- No requiere establecimiento de conexión.
- Mantiene límites de mensaje por datagrama.
- Tiene poca sobrecarga conceptual y de encabezado.
- Permite a la aplicación decidir su propia política de pérdida y repetición.
- Sirve como base para protocolos modernos como QUIC.

Limitaciones:

- No garantiza entrega.
- No garantiza orden.
- Puede duplicar datagramas.
- No evita que un mensaje grande exceda límites prácticos de transporte.
- No cifra ni autentica por sí mismo.
- No controla congestión de forma automática para la aplicación.

### Comparación con TCP y QUIC

TCP da un flujo confiable y ordenado de bytes. Es útil cuando perder un tramo
debe detener la entrega hasta reconstruir la secuencia.

UDP da datagramas independientes. Es útil cuando la aplicación prefiere decidir
qué hacer con pérdida, duplicación y latencia.

QUIC usa UDP como base, pero agrega seguridad, control de congestión,
establecimiento moderno y múltiples flujos. Dicho de otra forma: QUIC no usa UDP
porque quiera menos ingeniería, sino porque quiere controlar la ingeniería en
espacio de usuario.

## Diagramas

El diagrama principal vive en
[`diagrams/03-udp.mmd`](../diagrams/03-udp.mmd). Muestra datagramas
independientes y tres resultados posibles: entrega, duplicación o pérdida.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `UdpEndpoint::new` | O(a) | O(a) | O(a) | O(a) |
| `UdpDatagram::new` | O(1) | O(1) | O(1) | O(p) |
| `UdpDatagram::payload` | O(1) | O(1) | O(1) | O(1) |
| `DeliveryOutcome::deterministic` con entrega | O(1) | O(1) | O(1) | O(1) |
| `DeliveryOutcome::deterministic` con duplicación | O(p) | O(p) | O(p) | O(p) |

`a` es la longitud de la dirección textual y `p` el tamaño de la carga útil.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura debería permitir enviar una serie de
datagramas numerados y ver cuáles llegan, cuáles se duplican y cuáles se pierden.

## Implementación

La implementación define:

- `UdpEndpoint`: dirección textual y puerto;
- `UdpDatagram`: origen, destino y carga útil validada;
- `DeliveryOutcome`: entrega, pérdida o duplicación;
- `UdpError`: error por carga útil demasiado grande.

El modelo no usa sockets reales. Esto permite enseñar el contrato de UDP sin
depender del sistema operativo, privilegios locales o una red externa.

## Pruebas

Las pruebas cubren:

- datagrama con origen, destino y carga útil;
- modelo determinista de entrega, duplicación y pérdida;
- rechazo de carga útil por encima del límite.

## Benchmarks

El benchmark manual vive en
[`benches/udp_bench.rs`](../benches/udp_bench.rs). Mide creación de datagramas
pequeños, rechazo de datagramas demasiado grandes y despacho determinista de
entregas.

El objetivo es observar que validar tamaño es barato, mientras que duplicar
datagramas copia la carga útil y por tanto escala con su tamaño.

## Ejercicios

### Ejercicio 1: Crear un datagrama `[Nivel 1]`

Crea un datagrama desde `sensor-a:4000` hacia `colector:8125` con carga útil
`temperatura=31`.

**Entrada/Salida esperada:** `payload()` devuelve `temperatura=31`.

<details>
<summary>Pista</summary>
Usa `UdpEndpoint::new` para origen y destino antes de crear el datagrama.
</details>

### Ejercicio 2: Modelar pérdida `[Nivel 2]`

Usa `DeliveryOutcome::deterministic` con secuencia 5.

**Entrada/Salida esperada:** el resultado es `DeliveryOutcome::Lost`.

<details>
<summary>Pista</summary>
En este modelo, los múltiplos de 5 se pierden.
</details>

### Ejercicio 3: Validar tamaño máximo `[Nivel 3]`

Intenta crear un datagrama con `UdpDatagram::MAX_PAYLOAD_SIZE + 1` bytes.

**Entrada/Salida esperada:** se devuelve `UdpError::PayloadTooLarge`.

<details>
<summary>Pista</summary>
No necesitas llenar el vector con datos reales; basta con ceros.
</details>

### Ejercicio 4: Telemetría tolerante a pérdida `[Nivel 4]`

Diseña una estrategia para enviar métricas periódicas por UDP sin romper el
sistema cuando algunos datagramas se pierdan o dupliquen.

<details>
<summary>Pista</summary>
Incluye marca de tiempo, identificador de sensor y decisión explícita sobre
duplicados.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/udp_datagram.rs`](../examples/soluciones/udp_datagram.rs)
- [`examples/soluciones/udp_loss_model.rs`](../examples/soluciones/udp_loss_model.rs)
- [`examples/soluciones/udp_size_limit.rs`](../examples/soluciones/udp_size_limit.rs)

Para el nivel 4, una respuesta sana empieza por diseñar mensajes idempotentes,
incluir identificadores o marcas de tiempo y aceptar que una lectura nueva puede
reemplazar una lectura perdida.

## Referencias

- Andrew S. Tanenbaum y David J. Wetherall, *Computer Networks*.
- James F. Kurose y Keith W. Ross, *Computer Networking: A Top-Down Approach*.
- RFC 768: *User Datagram Protocol*.
- RFC 8085: *UDP Usage Guidelines*.
