# HTTP

> **Curso:** rust-networking · **Capítulo:** 06 · **Prerrequisitos:** TCP, DNS
> y TLS conceptual
> **Código:** [`src/http.rs`](../src/http.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

HTTP es el protocolo que da semántica a gran parte de la web. TCP puede mover
bytes, TLS puede protegerlos y DNS puede encontrar un nombre, pero HTTP dice qué
quiere hacer una aplicación: leer un recurso, crear algo, enviar un cuerpo,
recibir un código de estado o reutilizar una respuesta en caché.

Este capítulo modela HTTP como contrato de mensajes. No construye un servidor
real ni implementa todo el estándar; muestra la anatomía mínima para razonar con
claridad: línea inicial, método, ruta, versión, encabezados, cuerpo, código de
estado y metadatos de caché.

## Motivación

Muchas fallas de APIs no nacen en el socket. Nacen en mensajes ambiguos:
métodos incorrectos, encabezados mal interpretados, cuerpos sin significado,
códigos de estado usados como decoración o caché que sirve contenido obsoleto.

La idea central es:

```text
HTTP convierte bytes en una conversación con intención, metadatos y resultado.
```

## Teoría

### Historia

HTTP nació como un protocolo sencillo para pedir documentos enlazados. Con el
tiempo se volvió la base de APIs, navegadores, descargas, formularios,
servicios internos y comunicación entre sistemas. HTTP/1.1 consolidó conexiones
persistentes y encabezados expresivos; HTTP/2 agregó multiplexación sobre una
conexión; HTTP/3 llevó esas ideas sobre QUIC.

### Fundamentos

Una solicitud HTTP tiene una línea inicial, encabezados y opcionalmente cuerpo:

```text
GET /academy HTTP/1.1
Host: jeresoft.test
Accept: text/plain
```

La línea inicial declara método, ruta y versión. Los encabezados agregan
metadatos: tipo de contenido, autenticación, caché, negociación de formatos,
longitud del cuerpo o identidad del host. El cuerpo transporta datos de la
aplicación cuando el método y el contrato lo permiten.

Una respuesta tiene versión, código de estado, frase legible, encabezados y
cuerpo. El código de estado no es un mensaje humano; es una señal operacional:
`200` significa éxito, `201` creación, `400` solicitud inválida, `404` recurso
inexistente y `500` falla interna.

### Solicitud y respuesta

HTTP es asimétrico: el cliente inicia la solicitud y el servidor responde. Esa
forma simple permite intermediarios, balanceadores, cachés, navegadores,
clientes móviles y pruebas reproducibles.

El modelo de este crate se concentra en:

- `HttpRequest`: método, ruta, versión, encabezados y cuerpo.
- `HttpResponse`: código de estado, encabezados y cuerpo.
- `HeaderMap`: mapa de encabezados con búsqueda insensible a mayúsculas.
- `StatusCode`: códigos frecuentes para enseñar intención.
- `HttpParseError`: errores explícitos de mensajes mal formados.

### Métodos

Los métodos expresan intención. `GET` consulta, `POST` envía una representación,
`PUT` reemplaza, `PATCH` modifica parcialmente, `DELETE` elimina, `HEAD`
consulta metadatos y `OPTIONS` pregunta capacidades.

El método por sí solo no basta. La semántica real surge del contrato entre
cliente y servidor: ruta, autorización, cuerpo aceptado, código de estado y
garantías de idempotencia.

### Encabezados

Los encabezados son metadatos. Este capítulo usa claves normalizadas a
minúsculas para que `Host`, `host` y `HOST` representen el mismo campo.

Encabezados importantes:

- `Host`: autoridad solicitada.
- `Content-Type`: formato del cuerpo enviado.
- `Accept`: formato preferido en la respuesta.
- `Cache-Control`: política de reutilización.
- `ETag`: identificador de versión de una representación.

### Cuerpo

El cuerpo pertenece a la aplicación. HTTP no sabe si transporta JSON, texto,
HTML o bytes binarios; eso se comunica con encabezados y contrato. En este
modelo el cuerpo es `Vec<u8>` para no fingir que todo contenido es texto.

### Conexiones persistentes

En HTTP/1.1 una conexión TCP puede reutilizarse para varias solicitudes. Esto
reduce el costo de abrir conexiones, pero exige leer correctamente límites de
mensajes. En servidores reales importan `Content-Length`, transferencia por
fragmentos, tiempos de espera y cierre ordenado.

Este capítulo no implementa transporte ni concurrencia. Esos temas se conectan
con `rust-operating-systems`, `rust-async` y capítulos posteriores.

### Caché

La caché reduce latencia y carga, pero puede servir datos viejos si la política
es incorrecta. `Cache-Control` describe reglas como `max-age=60`. `ETag`
permite preguntar si una representación cambió sin descargarla completa.

El modelo no implementa una caché completa; solo conserva esos metadatos para
que los ejercicios puedan razonar sobre ellos.

### Casos de uso

HTTP aparece en:

- páginas web y recursos estáticos;
- APIs públicas y privadas;
- integraciones entre servicios;
- clientes móviles;
- balanceadores y proxies;
- sistemas de caché;
- observabilidad de tráfico de aplicación.

### Ventajas y limitaciones

Ventajas:

- Es legible y ampliamente interoperable.
- Se integra con cachés, proxies y herramientas de diagnóstico.
- Permite contratos claros de solicitud/respuesta.
- Se compone naturalmente con TLS para HTTPS.

Limitaciones:

- HTTP/1.1 puede sufrir bloqueo por orden en una conexión.
- Los encabezados pueden crecer y duplicar metadatos.
- La semántica se maltrata si todo se modela como `POST`.
- No reemplaza contratos estrictos cuando se necesitan esquemas fuertes.
- Este crate no implementa un parser completo ni endurecido para producción.

### Comparación con HTTP/2 y gRPC

HTTP/1.1 es fácil de inspeccionar y enseñar porque sus mensajes son texto con
límites sencillos. Es excelente para entender método, ruta, encabezados y
códigos de estado.

HTTP/2 conserva semántica HTTP, pero cambia el transporte de mensajes: usa
tramas binarias, compresión de encabezados y multiplexación. Resuelve varias
limitaciones de rendimiento, pero es menos transparente a simple vista.

gRPC normalmente usa HTTP/2 como transporte y agrega contratos de servicio,
mensajes tipados y distintos modos de flujo. Es útil para comunicación interna
con contratos estrictos, pero no elimina la necesidad de entender HTTP: se apoya
en sus códigos, metadatos, autoridad y transporte.

## Diagramas

El diagrama principal vive en
[`diagrams/06-http.mmd`](../diagrams/06-http.mmd). Muestra solicitud,
validación de encabezados, construcción de respuesta y metadatos de caché.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `HeaderMap::insert` | O(log n) | O(log n) | O(log n) | O(k + v) |
| `HeaderMap::get` | O(log n) | O(log n) | O(log n) | O(k) |
| `HttpRequest::parse` | O(l + h) | O(l + h) | O(l + h) | O(h + b) |
| `HttpResponse::with_body` | O(b) | O(b) | O(b) | O(b) |

`n` es el número de encabezados, `k` el tamaño del nombre, `v` el tamaño del
valor, `l` el tamaño de las líneas de encabezado, `h` la cantidad de
encabezados y `b` el tamaño del cuerpo.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría permitir cambiar método,
ruta, encabezados y código de estado para observar cómo cambia la respuesta.

## Implementación

La implementación define:

- `HttpMethod`: métodos comunes.
- `HttpVersion`: versiones `HTTP/1.0`, `HTTP/1.1` y `HTTP/2`.
- `HeaderMap`: encabezados normalizados.
- `HttpRequest`: parser limitado de solicitudes.
- `HttpResponse`: constructor de respuestas.
- `StatusCode`: códigos frecuentes y frase asociada.
- `HttpParseError`: errores educativos.

El parser acepta una solicitud textual pequeña, separa cabeza y cuerpo, valida
línea inicial y procesa encabezados con `:`. Rechaza métodos desconocidos,
versiones desconocidas, líneas iniciales incompletas y encabezados sin nombre.

## Pruebas

Las pruebas cubren:

- parseo de solicitud `GET` simple;
- rechazo de método inválido;
- construcción de respuesta con código, encabezados y cuerpo;
- helpers de caché para `Cache-Control` y `ETag`.

## Benchmarks

El benchmark manual vive en
[`benches/http_bench.rs`](../benches/http_bench.rs). Mide parseo de solicitudes,
rechazo de métodos inválidos y construcción de respuestas.

## Ejercicios

### Ejercicio 1: Parsear una solicitud `[Nivel 1]`

Parsea una solicitud `GET /academy HTTP/1.1` con encabezado `Host` y muestra la
ruta.

**Entrada/Salida esperada:** la ruta debe ser `/academy`.

<details>
<summary>Pista</summary>
Usa `HttpRequest::parse` y después `path`.
</details>

### Ejercicio 2: Construir una respuesta `[Nivel 2]`

Crea una respuesta `200 OK` con `Content-Type: text/plain` y cuerpo `hola`.

**Entrada/Salida esperada:** el código numérico debe ser `200` y el cuerpo debe
contener `hola`.

<details>
<summary>Pista</summary>
Encadena `HttpResponse::new`, `with_header` y `with_body`.
</details>

### Ejercicio 3: Agregar metadatos de caché `[Nivel 3]`

Crea encabezados con `Cache-Control: max-age=60` y `ETag: "lesson-1"`.

**Entrada/Salida esperada:** ambos valores deben poder leerse desde el mapa.

<details>
<summary>Pista</summary>
Usa `HeaderMap::new().with_cache_control(...).with_etag(...)`.
</details>

### Ejercicio 4: Elegir código de estado `[Nivel 4]`

Explica cuándo usarías `400`, `404` y `500` en una API educativa. No escribas
código; justifica cada caso con una falla distinta.

<details>
<summary>Pista</summary>
Distingue error del cliente, recurso inexistente y falla interna del servidor.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/http_parse_request.rs`](../examples/soluciones/http_parse_request.rs)
- [`examples/soluciones/http_status_response.rs`](../examples/soluciones/http_status_response.rs)
- [`examples/soluciones/http_cache_headers.rs`](../examples/soluciones/http_cache_headers.rs)

Para el nivel 4, una respuesta sana separa responsabilidad. `400` representa
entrada inválida, `404` representa ausencia de recurso y `500` representa una
falla que el servidor no pudo manejar correctamente.
