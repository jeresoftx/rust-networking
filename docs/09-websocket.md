# WebSocket

> **Curso:** rust-networking · **Capítulo:** 09 · **Prerrequisitos:** HTTP,
> HTTPS y nociones de aplicaciones interactivas
> **Código:** [`src/websocket.rs`](../src/websocket.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

WebSocket permite convertir una solicitud HTTP inicial en una conexión
persistente de mensajes bidireccionales. HTTP sigue siendo la puerta de entrada:
el cliente pide una actualización y, si el servidor acepta, la conversación deja
de ser solicitud/respuesta tradicional.

Este capítulo modela la parte estructural: actualización desde HTTP, tramas,
mensajes de texto, ping/pong y cierre ordenado. No implementa el formato binario
completo del estándar ni sockets reales.

## Motivación

Algunas aplicaciones necesitan recibir información sin esperar a que el usuario
vuelva a pedirla: notificaciones, pizarras colaborativas, chats, presencia,
telemetría ligera o paneles en vivo. Repetir solicitudes HTTP puede funcionar,
pero genera latencia, carga y complejidad.

La idea central es:

```text
WebSocket empieza como HTTP y después mantiene una conversación viva.
```

## Teoría

### Historia

Antes de WebSocket, muchas aplicaciones simulaban tiempo real con consultas
periódicas o técnicas de conexión sostenida. WebSocket formalizó un canal
bidireccional sobre una conexión persistente, útil para navegadores y servicios
que necesitan baja latencia.

### Fundamentos

El cliente inicia con una solicitud HTTP `GET` que incluye encabezados como:

```text
Connection: Upgrade
Upgrade: websocket
Sec-WebSocket-Key: ...
```

Si el servidor acepta, la conexión cambia de protocolo. A partir de ahí se
intercambian tramas con un `opcode` y una carga útil. Algunas tramas llevan
datos, como texto o binario. Otras son de control, como `ping`, `pong` y
`close`.

### Actualización desde HTTP

La actualización no es decorativa. Evita que cualquier solicitud HTTP se trate
como WebSocket por accidente. En este crate se validan tres cosas:

- el método debe ser `GET`;
- `Connection` debe contener `Upgrade`;
- `Upgrade` debe ser `websocket`;
- debe existir `Sec-WebSocket-Key`.

En producción hay más validaciones, incluyendo respuesta `Sec-WebSocket-Accept`,
origen, seguridad, límites y autenticación.

### Tramas

Una trama agrupa un código de operación y una carga útil. El modelo incluye:

- `Text`: mensaje de texto;
- `Binary`: bytes arbitrarios;
- `Ping`: control para verificar vida de la conexión;
- `Pong`: respuesta a `Ping`;
- `Close`: cierre ordenado.

Este crate no implementa máscaras, fragmentación, compresión ni codificación
binaria real. Es un modelo para razonar sobre semántica.

### Mensajes

Una aplicación normalmente piensa en mensajes, no en tramas. Una trama de texto
puede representar un mensaje completo en el modelo educativo. En producción, un
mensaje puede estar fragmentado en varias tramas.

### Ping/Pong

`Ping` y `Pong` ayudan a detectar conexiones vivas. Un ping debería generar un
pong con la misma carga útil. Esto permite distinguir una conexión abierta en el
sistema operativo de una conexión útil para la aplicación.

### Cierre

El cierre ordenado evita que ambos lados interpreten bytes restantes como datos
válidos. Un cierre normal usa código `1000`. Otros códigos comunican salida del
servicio o error de protocolo.

### Casos de uso

WebSocket aparece en:

- chats;
- notificaciones;
- colaboración en tiempo real;
- paneles de operación;
- juegos sencillos;
- presencia de usuarios;
- transmisión ligera de eventos de aplicación.

### Ventajas y limitaciones

Ventajas:

- Reduce latencia frente a consultas periódicas.
- Permite comunicación bidireccional.
- Reutiliza HTTP para iniciar la conexión.
- Encaja bien con navegadores y servicios interactivos.

Limitaciones:

- Mantener conexiones abiertas consume recursos.
- Requiere cuidado con autenticación, reconexión y límites.
- Puede complicar balanceadores, proxies y despliegues.
- No sustituye colas durables ni transmisión masiva de datos.
- Este crate no implementa codificación binaria de producción.

### Comparación contra polling y Server-Sent Events

Polling consulta cada cierto tiempo aunque no haya datos nuevos. Es simple, pero
puede desperdiciar recursos y aumentar latencia.

Server-Sent Events mantiene un canal del servidor al cliente. Es más simple que
WebSocket cuando solo se necesitan eventos unidireccionales.

WebSocket conviene cuando ambos lados deben enviar mensajes mientras la conexión
permanece abierta. A cambio, exige más disciplina operativa.

## Diagramas

El diagrama principal vive en
[`diagrams/09-websocket.mmd`](../diagrams/09-websocket.mmd). Muestra la
actualización HTTP, apertura, tramas, ping/pong y cierre.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `WebSocketUpgrade::accept` | O(h) | O(h) | O(h) | O(p) |
| `WebSocketFrame::text` | O(n) | O(n) | O(n) | O(n) |
| `WebSocketFrame::respond_to_ping` | O(n) | O(n) | O(n) | O(n) |
| `WebSocketConnection::apply_frame` | O(1) | O(n) para guardar datos | O(n) | O(n) |

`h` representa búsquedas de encabezados, `p` la ruta actualizada y `n` el tamaño
de la carga útil.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar cómo una solicitud
HTTP se convierte en conexión abierta y cómo cambian los estados al recibir
tramas.

## Implementación

La implementación define:

- `WebSocketUpgrade`: validación educativa de actualización.
- `WebSocketConnection`: conexión abierta y tramas recibidas.
- `WebSocketFrame`: trama con opcode y carga útil.
- `Opcode`: texto, binario, ping, pong y cierre.
- `CloseCode`: códigos frecuentes de cierre.
- `WebSocketState`: estados educativos.
- `WebSocketError`: errores de actualización y conexión cerrada.

El modelo responde `Ping` con `Pong`, almacena tramas de texto/binarias y cambia
a `Closed` cuando recibe una trama de cierre.

## Pruebas

Las pruebas cubren:

- actualización válida desde HTTP;
- trama de texto;
- ping/pong con la misma carga útil;
- cierre ordenado;
- rechazo de actualización incompleta.

## Benchmarks

El benchmark manual vive en
[`benches/websocket_bench.rs`](../benches/websocket_bench.rs). Mide validación
de actualización, creación de tramas y ping/pong.

## Ejercicios

### Ejercicio 1: Aceptar actualización `[Nivel 1]`

Crea una solicitud HTTP con encabezados de actualización y conviértela en una
conexión WebSocket abierta.

**Entrada/Salida esperada:** el estado debe ser `Open`.

<details>
<summary>Pista</summary>
Usa `WebSocketUpgrade::new(request).accept()`.
</details>

### Ejercicio 2: Crear tramas `[Nivel 2]`

Crea una trama de texto y una trama binaria. Verifica sus opcodes y cargas
útiles.

**Entrada/Salida esperada:** la trama de texto usa `Opcode::Text`.

<details>
<summary>Pista</summary>
`WebSocketFrame::text` convierte texto a bytes.
</details>

### Ejercicio 3: Ping/Pong `[Nivel 3]`

Crea un ping con carga útil `latido` y genera su pong.

**Entrada/Salida esperada:** el pong conserva la misma carga útil.

<details>
<summary>Pista</summary>
Usa `respond_to_ping`.
</details>

### Ejercicio 4: Elegir transporte `[Nivel 4]`

Compara WebSocket, polling y Server-Sent Events para un panel de notificaciones.
Elige uno y justifica el intercambio.

<details>
<summary>Pista</summary>
Pregunta si el cliente también necesita enviar mensajes frecuentes.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/websocket_upgrade.rs`](../examples/soluciones/websocket_upgrade.rs)
- [`examples/soluciones/websocket_frames.rs`](../examples/soluciones/websocket_frames.rs)
- [`examples/soluciones/websocket_ping_pong.rs`](../examples/soluciones/websocket_ping_pong.rs)

Para el nivel 4, una respuesta sana elige WebSocket cuando hay ida y vuelta
frecuente, Server-Sent Events cuando solo el servidor emite eventos y polling
cuando la simplicidad pesa más que la latencia.
