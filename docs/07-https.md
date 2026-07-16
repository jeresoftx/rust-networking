# HTTPS

> **Curso:** rust-networking · **Capítulo:** 07 · **Prerrequisitos:** TLS
> (capítulo 05) y HTTP (capítulo 06)
> **Código:** [`src/https.rs`](../src/https.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

HTTPS es HTTP transportado sobre TLS. La frase es corta, pero la decisión de
ingeniería es enorme: la aplicación conserva la semántica de HTTP y agrega una
capa de identidad, confidencialidad e integridad antes de enviar la solicitud.

Este capítulo no reimplementa HTTP ni TLS. Usa los modelos de los capítulos 05
y 06 para estudiar la composición: una solicitud HTTP solo se considera segura
cuando la negociación TLS valida la autoridad esperada.

## Motivación

HTTP plano permite leer y modificar mensajes en redes no confiables. TLS puede
negociar seguridad, pero por sí solo no expresa rutas, métodos ni códigos de
estado. HTTPS une ambos contratos: primero establece confianza en el canal,
luego transporta intención de aplicación.

La idea central es:

```text
HTTPS no es un protocolo nuevo de aplicación; es HTTP con transporte autenticado.
```

## Teoría

### Historia

HTTPS nació para proteger transacciones sensibles y terminó convirtiéndose en
el comportamiento esperado de la web. Hoy se usa para páginas públicas, APIs,
servicios internos, descargas, autenticación y comunicación entre sistemas.

El cambio cultural fue importante: cifrar dejó de ser una excepción para sitios
"sensibles" y se volvió la base normal de operación.

### Fundamentos

Una conexión HTTPS combina tres ideas:

- autoridad: el nombre que el cliente espera contactar;
- TLS: negociación de identidad y parámetros seguros;
- HTTP: mensaje de aplicación que viaja sobre el canal protegido.

En este crate, `HttpsRequest` une autoridad y `HttpRequest`. `SecureTransport`
intenta negociar TLS con `TlsHandshake`. Si la autoridad no coincide con el
nombre solicitado en TLS, o si TLS rechaza el certificado, la solicitud no se
considera transportable.

### Autoridad y certificados

La autoridad es el nombre que aparece en la URL, por ejemplo
`api.jeresoft.test`. Ese nombre debe coincidir con la identidad del certificado
presentado por el servidor. Si el certificado pertenece a
`otro.jeresoft.test`, el cliente debe rechazar la conexión aunque el servidor
responda algo útil.

Este capítulo modela la propagación de errores TLS. No convierte errores de
certificado en errores HTTP: si la identidad falla, todavía no hay conversación
HTTP confiable.

### HSTS

HSTS significa HTTP Strict Transport Security. Es una política que le dice al
cliente: "para este host, usa HTTPS de forma obligatoria". Su valor principal es
evitar degradaciones accidentales o maliciosas hacia HTTP plano.

El modelo educativo conserva dos decisiones:

- una lista de hosts que deben usar HTTPS;
- una opción para incluir subdominios.

No implementa expiración, precarga de navegador ni almacenamiento persistente.

### Errores comunes

Errores frecuentes:

- certificado emitido para otro nombre;
- cadena incompleta;
- suite criptográfica obsoleta;
- redirección a HTTP después de una visita segura;
- HSTS mal aplicado a subdominios que todavía no soportan HTTPS;
- confundir `200 OK` con conexión segura.

Un código HTTP exitoso solo importa después de que el canal seguro fue aceptado.

### Casos de uso

HTTPS aparece en:

- APIs públicas;
- paneles administrativos;
- autenticación;
- pagos;
- integraciones entre servicios;
- descargas verificables por canal;
- comunicación de clientes móviles;
- sitios educativos y documentación.

### Ventajas y limitaciones

Ventajas:

- Protege confidencialidad e integridad del mensaje HTTP.
- Autentica identidad del servidor cuando TLS está bien configurado.
- Conserva compatibilidad con semántica HTTP.
- Permite políticas de transporte como HSTS.

Limitaciones:

- No corrige una API mal diseñada.
- No reemplaza autorización ni validación de entrada.
- Depende de certificados, suites y configuración correctas.
- HSTS mal aplicado puede bloquear hosts legítimos.
- Este crate no implementa criptografía, sockets ni redirecciones reales.

### Comparación con HTTP plano y TLS mal configurado

HTTP plano es fácil de inspeccionar, pero expone encabezados, rutas y cuerpos.
Puede ser aceptable dentro de ejercicios locales, no como transporte normal en
redes reales.

HTTPS bien configurado protege el canal antes de transportar HTTP. La aplicación
sigue necesitando buenas decisiones de dominio, pero al menos la conversación
viaja con identidad y confidencialidad.

TLS mal configurado es una falsa calma: un certificado para otro nombre, una
cadena rota o una suite obsoleta pueden dejar al sistema en un estado inseguro.
Por eso este modelo propaga errores TLS en vez de esconderlos.

## Diagramas

El diagrama principal vive en
[`diagrams/07-https.mmd`](../diagrams/07-https.mmd). Muestra autoridad, HSTS,
negociación TLS y transporte de una solicitud HTTP.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `HttpsRequest::new` | O(a) | O(a) | O(a) | O(a) |
| `HstsPolicy::should_force_https` | O(h) | O(h * l) | O(h * l) | O(1) |
| `SecureTransport::connect` | O(1) + TLS | O(TLS) | O(TLS) | O(1) |

`a` es el tamaño de la autoridad, `h` la cantidad de hosts registrados, `l` el
tamaño promedio de nombres y `TLS` representa la complejidad estudiada en el
capítulo 05.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría cambiar certificado,
autoridad y política HSTS para mostrar por qué una conexión se acepta o rechaza.

## Implementación

La implementación define:

- `HttpsRequest`: autoridad y solicitud HTTP.
- `HstsPolicy`: hosts obligados a HTTPS y cobertura de subdominios.
- `HttpsPolicy`: agrupación de política de transporte seguro.
- `SecureTransport`: solicitud HTTP aceptada sobre negociación TLS válida.
- `HttpsError`: errores de autoridad y errores propagados desde TLS.

La función central es `SecureTransport::connect`. Primero revisa que la
autoridad de la solicitud coincida con el nombre esperado por TLS. Después llama
a `TlsHandshake::negotiate`. Si TLS falla, el error se conserva.

## Pruebas

Las pruebas cubren:

- composición de solicitud HTTP sobre sesión TLS válida;
- propagación de falla de identidad de certificado;
- política HSTS para host exacto y subdominio.

## Benchmarks

El benchmark manual vive en
[`benches/https_bench.rs`](../benches/https_bench.rs). Mide composición segura,
rechazo por certificado y evaluación de política HSTS.

## Ejercicios

### Ejercicio 1: Solicitud segura `[Nivel 1]`

Crea una solicitud `GET /academy` para `api.jeresoft.test` y negocia TLS con un
certificado para ese mismo nombre.

**Entrada/Salida esperada:** el transporte devuelve `TlsVersion::V1_3`.

<details>
<summary>Pista</summary>
La autoridad de `HttpsRequest` y el nombre del `TlsClientHello` deben coincidir.
</details>

### Ejercicio 2: Rechazar certificado incorrecto `[Nivel 2]`

Intenta transportar una solicitud para `api.jeresoft.test` usando un certificado
de hoja para `otro.jeresoft.test`.

**Entrada/Salida esperada:** el resultado debe ser `HttpsError::Tls`.

<details>
<summary>Pista</summary>
No conviertas el error TLS en código HTTP. La sesión segura nunca se aceptó.
</details>

### Ejercicio 3: Política HSTS `[Nivel 3]`

Crea una política que fuerce HTTPS para `api.jeresoft.test` y sus subdominios.
Verifica que `v1.api.jeresoft.test` también quede cubierto.

**Entrada/Salida esperada:** ambos hosts deben devolver `true`.

<details>
<summary>Pista</summary>
Usa `include_host` seguido de `include_subdomains`.
</details>

### Ejercicio 4: Riesgo operativo `[Nivel 4]`

Explica por qué habilitar HSTS con subdominios puede romper un sistema si no
todos los subdominios ya soportan HTTPS correctamente.

<details>
<summary>Pista</summary>
Piensa en clientes que obedecen HSTS antes de intentar HTTP plano.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/https_secure_request.rs`](../examples/soluciones/https_secure_request.rs)
- [`examples/soluciones/https_certificate_failure.rs`](../examples/soluciones/https_certificate_failure.rs)
- [`examples/soluciones/https_hsts_policy.rs`](../examples/soluciones/https_hsts_policy.rs)

Para el nivel 4, una respuesta sana reconoce que HSTS es una promesa operativa:
si se activa para subdominios, todos esos nombres deben estar preparados para
servir HTTPS con certificados válidos.
