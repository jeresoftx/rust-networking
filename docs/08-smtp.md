# SMTP

> **Curso:** rust-networking · **Capítulo:** 08 · **Prerrequisitos:** DNS,
> TCP y fundamentos de HTTP/HTTPS como contraste
> **Código:** [`src/smtp.rs`](../src/smtp.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

SMTP es el protocolo clásico para transferir correo electrónico entre sistemas.
No está diseñado como una API moderna de producto; está diseñado como una
conversación entre servidores que aceptan, encaminan o rechazan mensajes.

Este capítulo modela cuatro ideas que suelen confundirse: sesión SMTP, sobre
del mensaje, encabezados visibles y selección de servidores MX. El objetivo no
es enviar correo real, sino entender qué contrato sostiene la entrega.

## Motivación

El correo parece simple desde una interfaz: escribir destinatario, asunto y
cuerpo. Por debajo hay más capas: DNS descubre servidores MX, SMTP negocia una
sesión, el sobre define entrega y los encabezados representan lo que verá una
persona o cliente de correo.

La idea central es:

```text
SMTP entrega un sobre; el mensaje visible viaja dentro, pero no son lo mismo.
```

## Teoría

### Historia

SMTP nació en un internet más pequeño y cooperativo. Con el tiempo tuvo que
convivir con autenticación, cifrado oportunista, reputación, listas negras,
filtros de spam, límites de tamaño y proveedores con políticas estrictas.

Aunque hoy muchas aplicaciones envían correo mediante APIs HTTP de proveedores,
SMTP sigue siendo la base conceptual de la entrega entre servidores.

### Fundamentos

Una sesión típica contiene:

```text
HELO cliente.example
MAIL FROM:<rebotes@example>
RCPT TO:<usuario@example>
DATA
Subject: Hola

Cuerpo del mensaje
.
```

`HELO` inicia la conversación. `MAIL FROM` define el remitente del sobre, que se
usa para rebotes. `RCPT TO` agrega destinatarios de entrega. `DATA` abre el
envío del contenido. Después viajan encabezados y cuerpo.

### Sesiones

SMTP es una máquina de estados. Algunos comandos solo tienen sentido después de
otros. `MAIL FROM` antes de `HELO` debe rechazarse; `DATA` antes de al menos un
destinatario tampoco es una conversación válida.

El modelo de este crate implementa una sesión pequeña:

- espera `HELO`;
- acepta `MAIL FROM`;
- acepta uno o más `RCPT TO`;
- abre `DATA`;
- guarda el mensaje;
- permite iniciar otro mensaje en la misma sesión.

### Sobre del mensaje

El sobre SMTP contiene información de transporte:

- remitente de rebote;
- destinatarios reales de entrega.

Ese sobre puede diferir de los encabezados visibles. Por ejemplo, el remitente
del sobre puede ser `bounce@jeresoft.test`, mientras que el encabezado `From`
muestra `Profesor <profesor@jeresoft.test>`.

### Encabezados

Los encabezados son parte del mensaje visible: `From`, `To`, `Subject`, `Date`,
`Message-ID` y otros. Ayudan a clientes de correo, filtros y personas, pero no
son lo mismo que el sobre usado por SMTP para entregar.

Este capítulo normaliza nombres de encabezados a minúsculas para facilitar
búsquedas sin depender de mayúsculas.

### MX

DNS usa registros MX para decir qué servidores aceptan correo de un dominio. La
prioridad numérica menor se intenta primero. Si falla, pueden intentarse
servidores de respaldo.

El modelo incluye `MxRecord` y `select_mx_by_priority` para enseñar selección
por prioridad sin hacer consultas DNS reales.

### Límites del correo electrónico

El correo electrónico no es mensajería instantánea garantizada. Puede diferirse,
reintentarse, filtrarse, rechazarse o reescribirse por políticas de entrega. Los
adjuntos y tamaños también tienen límites operativos.

Este crate no implementa autenticación, STARTTLS, DKIM, SPF, DMARC, colas
persistentes ni reintentos reales. Es un modelo estructural para entender el
protocolo antes de operar sistemas de correo.

### Casos de uso

SMTP aparece en:

- envío entre servidores de correo;
- notificaciones de sistemas;
- colas de correo saliente;
- rebotes y reportes de entrega;
- integraciones heredadas;
- diagnóstico de registros MX;
- aprendizaje de protocolos conversacionales.

### Ventajas y limitaciones

Ventajas:

- Modelo abierto y ampliamente interoperable.
- Permite entrega entre dominios distintos.
- Se apoya en DNS para descubrimiento.
- Separa transporte de contenido visible.

Limitaciones:

- La entrega puede ser diferida o rechazada.
- La seguridad moderna requiere capas y políticas adicionales.
- El spam hizo que la reputación sea parte central de operar correo.
- No es tan directo para aplicaciones modernas como una API HTTP de proveedor.
- Este crate no envía correo real.

### Comparación con APIs HTTP de envío de correo

SMTP es el protocolo de entrega entre servidores. Una API HTTP de proveedor
suele ofrecer una interfaz más cómoda para aplicaciones: plantillas,
estadísticas, autenticación simple, webhooks y errores más adaptados al dominio
del producto.

La API HTTP no elimina SMTP: normalmente el proveedor termina entregando por
SMTP o interactuando con infraestructura compatible. Para una aplicación, la
API puede ser más ergonómica; para entender entrega de correo, SMTP sigue siendo
el canon.

## Diagramas

El diagrama principal vive en
[`diagrams/08-smtp.mmd`](../diagrams/08-smtp.mmd). Muestra selección MX,
sesión SMTP, sobre, encabezados y aceptación del mensaje.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `SmtpSession::apply` | O(1) | O(h + b) para datos | O(h + b) | O(h + b) |
| `MailEnvelope::add_recipient` | O(r) | O(r) | O(r) | O(r) |
| `EmailHeaders::get` | O(log n) | O(log n) | O(log n) | O(k) |
| `select_mx_by_priority` | O(m) | O(m) | O(m) | O(1) |

`h` es el tamaño de encabezados, `b` el tamaño del cuerpo, `r` el tamaño del
destinatario, `n` la cantidad de encabezados, `k` el tamaño del nombre buscado y
`m` la cantidad de registros MX.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría permitir arrastrar comandos
SMTP para observar qué órdenes son válidas y cuáles se rechazan.

## Implementación

La implementación define:

- `SmtpCommand`: comandos educativos de sesión.
- `SmtpReply`: código y texto de respuesta.
- `MailEnvelope`: remitente de rebote y destinatarios.
- `EmailHeaders`: encabezados visibles del mensaje.
- `AcceptedMessage`: mensaje aceptado por la sesión.
- `MxRecord`: registro MX educativo.
- `SmtpSession`: máquina de estados.
- `SmtpError`: errores de secuencia y datos mínimos.

La máquina rechaza comandos fuera de orden con `UnexpectedCommand`, conserva
mensajes aceptados y permite iniciar otro mensaje después de aceptar datos.

## Pruebas

Las pruebas cubren:

- conversación `HELO`, `MAIL FROM`, `RCPT TO`, `DATA`;
- separación entre sobre y encabezados visibles;
- selección de MX por prioridad menor;
- rechazo de comandos fuera de orden.

## Benchmarks

El benchmark manual vive en
[`benches/smtp_bench.rs`](../benches/smtp_bench.rs). Mide conversación válida,
rechazo de comandos fuera de orden y selección de MX.

## Ejercicios

### Ejercicio 1: Conversación mínima `[Nivel 1]`

Crea una sesión y ejecuta `HELO`, `MAIL FROM`, `RCPT TO`, `DATA` y datos del
mensaje.

**Entrada/Salida esperada:** la sesión debe aceptar un mensaje.

<details>
<summary>Pista</summary>
Después de `DATA`, usa `SmtpCommand::MessageData`.
</details>

### Ejercicio 2: Sobre contra encabezados `[Nivel 2]`

Crea un sobre con remitente de rebote y encabezados con `From`, `To` y
`Subject`. Verifica que el remitente del sobre pueda diferir del encabezado
`From`.

**Entrada/Salida esperada:** ambos valores existen y no tienen que ser iguales.

<details>
<summary>Pista</summary>
`MailEnvelope` modela transporte; `EmailHeaders` modela contenido visible.
</details>

### Ejercicio 3: Elegir MX `[Nivel 3]`

Crea tres registros MX y selecciona el de menor prioridad numérica.

**Entrada/Salida esperada:** se elige el servidor con prioridad `10` si los
otros tienen prioridad `20` y `50`.

<details>
<summary>Pista</summary>
Usa `select_mx_by_priority`.
</details>

### Ejercicio 4: Entrega diferida `[Nivel 4]`

Explica por qué un servidor SMTP puede aceptar un mensaje y aun así no lograr
entregarlo inmediatamente.

<details>
<summary>Pista</summary>
Considera caídas temporales, reputación, límites y reintentos.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/smtp_conversation.rs`](../examples/soluciones/smtp_conversation.rs)
- [`examples/soluciones/smtp_envelope_headers.rs`](../examples/soluciones/smtp_envelope_headers.rs)
- [`examples/soluciones/smtp_mx_selection.rs`](../examples/soluciones/smtp_mx_selection.rs)

Para el nivel 4, una respuesta sana distingue aceptación local de entrega final:
el sistema puede poner el mensaje en cola y reintentar si el destino no está
disponible o si una política temporal lo retrasa.
