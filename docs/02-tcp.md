# TCP

> **Curso:** rust-networking · **Capítulo:** 02 · **Prerrequisitos:** modelo de
> capas, IP, enrutamiento, Rust básico y manejo de `Result`
> **Código:** [`src/tcp.rs`](../src/tcp.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

TCP existe porque IP no promete orden, entrega ni ausencia de duplicados. IP
intenta mover paquetes; TCP construye una conversación confiable entre dos
procesos encima de ese camino incierto.

Este capítulo modela las invariantes esenciales: establecimiento en tres pasos,
números de secuencia, acuses, reordenamiento, retransmisión y cierre con `FIN`.
No implementa una pila de red real ni sockets del sistema operativo. El objetivo
es que puedas leer trazas, entender fallas de conexión y reconocer qué contrato
está dando TCP antes de llegar a HTTP, TLS, WebSocket, gRPC o QUIC.

## Motivación

Una aplicación suele pensar en mensajes completos: "envía esta solicitud" o
"recibe esta respuesta". La red no trabaja así. Los bytes cruzan un camino donde
pueden retrasarse, llegar en otro orden o perderse.

TCP responde con una abstracción: un flujo ordenado de bytes. Para lograrlo,
cada segmento lleva un número de secuencia y el receptor confirma hasta dónde ha
recibido datos contiguos. Si falta un tramo, el emisor conserva segmentos
pendientes de acuse para poder retransmitirlos.

La idea central es:

```text
TCP no hace confiable a IP; construye confiabilidad encima de IP.
```

## Teoría

### Historia

TCP nació junto con IP para separar dos responsabilidades. IP mueve paquetes
entre redes. TCP ofrece una conversación confiable entre procesos. Esa separación
permitió que aplicaciones como correo, web, transferencia de archivos y
conexiones remotas compartieran el mismo transporte sin reescribir control de
orden y pérdida en cada aplicación.

### Fundamentos

El establecimiento en tres pasos sincroniza números iniciales de secuencia:

```text
cliente -> servidor: SYN
servidor -> cliente: SYN+ACK
cliente -> servidor: ACK
```

Después de eso, cada byte ocupa una posición en el flujo. Un segmento con cinco
bytes desde la secuencia 10 cubre las posiciones 10 a 14; el siguiente byte
esperado es 15. En TCP real, `SYN` y `FIN` también consumen espacio de
secuencia. El modelo educativo conserva esa regla porque evita atajos mentales
incorrectos.

El receptor solo entrega datos contiguos a la aplicación. Si llega primero el
tramo 6..10 y todavía falta 1..5, el tramo tardío se guarda. Cuando llega el
hueco, ambos tramos pueden entregarse como un flujo ordenado.

La retransmisión existe porque el emisor no puede asumir que un segmento llegó.
Mientras un acuse no cubra un segmento enviado, ese segmento permanece en la
lista de pendientes.

El cierre ordenado usa `FIN`. Este capítulo modela el lado que inicia el cierre:
envía `FIN`, espera el acuse correspondiente y pasa a `Closed`.

### Casos de uso

Este modelo ayuda a:

- entender por qué una conexión puede fallar antes de enviar datos de aplicación;
- distinguir "no conecta" de "conecta pero no responde";
- leer trazas con `SYN`, `SYN+ACK`, `ACK` y `FIN`;
- explicar por qué TCP entrega bytes en orden aunque IP no lo haga;
- razonar sobre retransmisión, latencia y espera por acuses;
- preparar HTTP, TLS, HTTPS, WebSocket, gRPC y QUIC.

### Ventanas

TCP real usa ventanas para limitar cuántos bytes pueden estar en vuelo sin acuse.
La ventana evita saturar al receptor y permite aprovechar el camino sin esperar
un acuse por cada segmento.

El crate todavía no implementa una ventana deslizante completa. La menciona como
contrato conceptual y conserva la lista de segmentos pendientes para que el paso
hacia una ventana real sea natural.

### Ventajas y limitaciones

Ventajas:

- Entrega un flujo ordenado de bytes a la aplicación.
- Detecta pérdida mediante ausencia de acuses.
- Permite retransmitir segmentos pendientes.
- Establece y cierra conexiones de forma explícita.
- Oculta parte de la incertidumbre de IP a las capas superiores.

Limitaciones:

- No conserva límites de mensaje: entrega bytes, no "paquetes de aplicación".
- Agrega latencia por establecimiento, control y retransmisión.
- Puede sufrir bloqueo por huecos: un segmento perdido detiene la entrega de
  bytes posteriores hasta que se rellena el faltante.
- No cifra ni autentica por sí mismo; eso vive en TLS.
- No resuelve movilidad ni cambios de ruta tan bien como QUIC.

### Comparación con UDP y QUIC

UDP no establece conexión, no ordena y no retransmite. Es útil cuando la
aplicación prefiere controlar pérdida, latencia y formato de mensajes, como en
telemetría, voz, video o protocolos propios.

TCP ofrece una base confiable y madura. Es la opción natural para HTTP clásico,
bases de datos, colas sencillas y protocolos donde el orden completo importa más
que evitar esperas por huecos.

QUIC construye confiabilidad sobre UDP, integra seguridad moderna y permite
múltiples flujos independientes. Reduce algunos costos de establecimiento y evita
que un hueco en un flujo bloquee a todos los demás.

## Diagramas

El diagrama principal vive en
[`diagrams/02-tcp.mmd`](../diagrams/02-tcp.mmd). Muestra establecimiento,
entrega ordenada, retransmisión y cierre ordenado.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `SequenceNumber::advance` | O(1) | O(1) | O(1) | O(1) |
| `TcpConnection::open` | O(1) | O(1) | O(1) | O(1) adicional |
| `TcpConnection::receive` con acuse | O(n) | O(n) | O(n) | O(1) |
| `TcpConnection::receive` con datos en orden | O(1) | O(p log p) | O(p log p) | O(1) |
| `TcpConnection::receive` con datos fuera de orden | O(log p) | O(log p) | O(log p) | O(k) |
| `TcpConnection::send_data` | O(1) | O(1) | O(1) | O(k) |
| `TcpConnection::retransmit_unacked` | O(n) | O(n) | O(n) | O(n) |
| `TcpConnection::close` | O(1) | O(1) | O(1) | O(1) adicional |

`n` es el número de segmentos enviados sin acuse, `p` el número de segmentos
fuera de orden pendientes y `k` el tamaño de la carga útil copiada al buffer.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura debería permitir arrastrar segmentos
fuera de orden, mostrar el número de secuencia esperado y observar cuándo se
libera la entrega a la aplicación.

## Implementación

La implementación define:

- `SequenceNumber`: número de secuencia educativo;
- `TcpState`: estados principales del modelo;
- `TcpSegment`: banderas, acuse, número de secuencia y carga útil;
- `TcpConnection`: máquina de estados y buffers de una conexión;
- `TcpEvent`: eventos observables para ejemplos y pruebas;
- `TcpError`: errores de transición y retransmisión.

El modelo separa constructores de segmentos (`syn`, `syn_ack`, `ack`, `fin`,
`data`) de lectores booleanos (`is_syn`, `is_ack`, `is_fin`). Esa convención
evita ambigüedad en Rust y mantiene explícito que las banderas son propiedades
del segmento.

Para enviar datos desde una conexión establecida se usa `send_data`. El método
crea el segmento, avanza el número de secuencia y lo conserva como pendiente
hasta recibir un acuse que lo cubra.

## Pruebas

Las pruebas cubren:

- establecimiento en tres pasos hasta `Established`;
- rechazo de segmentos inesperados para el estado actual;
- buffer de segmentos fuera de orden hasta rellenar el hueco;
- retransmisión de segmentos sin acuse;
- cierre iniciado con `FIN` y confirmado con `ACK`.

## Benchmarks

El benchmark manual vive en
[`benches/tcp_bench.rs`](../benches/tcp_bench.rs). Mide la entrega de segmentos
en orden, la entrega después de reordenamiento y la copia de segmentos
pendientes para retransmisión.

El objetivo no es competir contra la pila TCP del sistema operativo. El objetivo
es observar que el reordenamiento necesita memoria adicional y que las copias de
retransmisión crecen con los segmentos pendientes.

## Ejercicios

### Ejercicio 1: Establecer una conexión `[Nivel 1]`

Crea un cliente con secuencia inicial 100 y un servidor con secuencia inicial
500. Ejecuta `SYN`, `SYN+ACK` y `ACK` hasta que ambos queden en `Established`.

**Entrada/Salida esperada:** ambos estados finales son `TcpState::Established`.

<details>
<summary>Pista</summary>
`open` inicia el lado cliente y `receive` devuelve la respuesta cuando el estado
lo requiere.
</details>

### Ejercicio 2: Entrega ordenada con hueco `[Nivel 2]`

Crea una conexión establecida que espera la secuencia 1. Recibe primero un
segmento con secuencia 6 y carga útil `world`; después recibe secuencia 1 con
carga útil `hello`.

**Entrada/Salida esperada:** la carga útil entregada es `helloworld`.

<details>
<summary>Pista</summary>
El segmento tardío se guarda hasta que llega el tramo que llena el hueco.
</details>

### Ejercicio 3: Retransmisión pendiente `[Nivel 3]`

Inicia una conexión cliente y conserva el `SYN`. Mientras no llegue un acuse,
solicita retransmisión de segmentos pendientes.

**Entrada/Salida esperada:** la lista de retransmisión contiene el `SYN`.

<details>
<summary>Pista</summary>
Un segmento permanece pendiente hasta que un acuse cubre su número final de
secuencia.
</details>

### Ejercicio 4: Diagnóstico de conexión `[Nivel 4]`

Explica cómo investigarías un caso donde una aplicación no puede conectar al
puerto 443, pero DNS resuelve correctamente y la ruta IP existe.

<details>
<summary>Pista</summary>
Separa fallo de establecimiento TCP, rechazo por firewall, servicio no
escuchando, TLS y respuesta HTTP.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/tcp_handshake.rs`](../examples/soluciones/tcp_handshake.rs)
- [`examples/soluciones/tcp_ordered_stream.rs`](../examples/soluciones/tcp_ordered_stream.rs)
- [`examples/soluciones/tcp_retransmission.rs`](../examples/soluciones/tcp_retransmission.rs)

Para el nivel 4, una respuesta sana empieza por confirmar intento de conexión al
puerto correcto, observar si hay `SYN+ACK` o rechazo, revisar reglas de red,
confirmar que el servicio escucha y solo después subir a TLS o HTTP.

## Referencias

- Andrew S. Tanenbaum y David J. Wetherall, *Computer Networks*.
- James F. Kurose y Keith W. Ross, *Computer Networking: A Top-Down Approach*.
- RFC 9293: *Transmission Control Protocol*.
- RFC 5681: *TCP Congestion Control*.
