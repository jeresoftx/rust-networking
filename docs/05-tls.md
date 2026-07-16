# TLS

> **Curso:** rust-networking · **Capítulo:** 05 · **Prerrequisitos:** TCP, DNS,
> HTTP conceptual y nociones básicas de identidad en sistemas distribuidos
> **Código:** [`src/tls.rs`](../src/tls.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

TLS existe para proteger una conversación sobre un transporte inseguro. En la
web moderna suele viajar encima de TCP y debajo de HTTP: TCP conecta procesos,
TLS negocia seguridad y HTTP expresa la semántica de la aplicación.

Este capítulo no implementa criptografía de producción. Modela el contrato de
ingeniería: identidad del servidor, cadena de certificados, versión, suite
criptográfica, política mínima de rechazo y resultado de negociación.

## Motivación

Resolver DNS y abrir TCP no basta para confiar en una conexión. Una aplicación
también necesita saber con quién habla, evitar texto plano y rechazar algoritmos
obsoletos. TLS agrupa esas decisiones en un protocolo de negociación.

La idea central es:

```text
TLS no solo cifra; también autentica identidad y negocia parámetros seguros.
```

## Teoría

### Historia

TLS reemplazó a SSL y se convirtió en el mecanismo estándar para proteger HTTP,
correo, APIs y muchos protocolos de aplicación. Su evolución ha reducido
algoritmos débiles, simplificado negociación y favorecido versiones modernas.

### Fundamentos

Un cliente envía un `ClientHello` con el nombre esperado, versiones soportadas y
suites criptográficas aceptables. El servidor responde con una versión, una
suite y una cadena de certificados.

El certificado de hoja identifica al servidor. La cadena conecta esa identidad
con emisores intermedios hasta una autoridad de confianza. En producción, esa
cadena implica firmas, fechas, usos de clave, revocación y raíces confiables.
Aquí solo se modela la estructura: el emisor de un certificado debe coincidir
con el sujeto del siguiente.

La negociación selecciona una versión común y una suite común permitida. Si solo
hay una suite obsoleta, se rechaza aunque ambos lados la conozcan.

### Seguridad Que Modela Este Capítulo

- Confidencialidad: terceros no deberían leer el contenido.
- Integridad: terceros no deberían modificar mensajes sin detección.
- Autenticación: el servidor debe demostrar identidad.
- Negociación: cliente y servidor acuerdan versión y suite.
- Política: algoritmos obsoletos se rechazan.

El crate no implementa cifrado, firmas, derivación de llaves ni validación real
de certificados. Es un modelo estructural para razonar antes de usar bibliotecas
reales como `rustls`.

### Casos de uso

Este modelo ayuda a:

- entender por qué un certificado válido para otro nombre debe rechazarse;
- diagnosticar cadenas incompletas;
- separar fallas TLS de fallas DNS, TCP o HTTP;
- explicar por qué no basta con "que conecte";
- preparar HTTPS, gRPC y QUIC;
- justificar el rechazo de algoritmos heredados.

### Ventajas y limitaciones

Ventajas:

- Agrega identidad a una conexión.
- Protege confidencialidad e integridad cuando se implementa con criptografía
  real.
- Permite negociar capacidades sin fijar una sola opción en código.
- Facilita políticas de seguridad evolutivas.

Limitaciones:

- Su seguridad real depende de implementación criptográfica correcta.
- Los certificados pueden estar mal emitidos, vencidos o incompletos.
- La negociación agrega complejidad operativa.
- Este crate no implementa criptografía de producción.

### Comparación con cifrado casero, VPN y texto plano

El texto plano es fácil de inspeccionar, pero no ofrece confidencialidad ni
integridad frente a una red no confiable.

El cifrado casero suele fallar en detalles difíciles: autenticación, modos de
cifrado, renovación de llaves, aleatoriedad y compatibilidad. Este curso lo
trata como una mala decisión para producción.

Una VPN protege un tramo de red, pero no reemplaza la identidad de aplicación
extremo a extremo. TLS sigue siendo necesario para APIs públicas y privadas.

## Diagramas

El diagrama principal vive en
[`diagrams/05-tls.mmd`](../diagrams/05-tls.mmd). Muestra identidad, cadena,
negociación de versión, selección de suite y rechazo por política.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `Certificate::new` | O(s + i) | O(s + i) | O(s + i) | O(s + i) |
| `CertificateChain::validate_structure` | O(c) | O(c) | O(c) | O(1) |
| selección de versión | O(v * s) | O(v * s) | O(v * s) | O(1) |
| selección de suite | O(c * s) | O(c * s) | O(c * s) | O(1) |
| `TlsHandshake::negotiate` | O(1) | O(c + v*s + a*b) | O(c + v*s + a*b) | O(1) |

`c` es el número de certificados, `v` versiones del cliente, `s` versiones del
servidor, `a` suites del cliente y `b` suites del servidor.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura debería permitir cambiar nombre de
servidor, certificados y suites para observar por qué se acepta o rechaza una
negociación.

## Implementación

La implementación define:

- `Certificate`: sujeto y emisor;
- `CertificateChain`: cadena con hoja primero;
- `TlsVersion`: versiones educativas TLS 1.2 y TLS 1.3;
- `CipherSuite`: suites modernas y una suite obsoleta para rechazo;
- `TlsClientHello`: nombre esperado, versiones y suites del cliente;
- `TlsServerHello`: versión, suite y certificado negociados;
- `TlsHandshake`: negociación estructural;
- `TlsError`: errores de identidad, cadena, versión y suite.

La implementación declara explícitamente que no cifra, no firma y no valida
certificados de producción.

## Pruebas

Las pruebas cubren:

- coincidencia de nombre del servidor;
- rechazo de cadena incompleta;
- selección de versión y suite común;
- rechazo de suite obsoleta.

## Benchmarks

El benchmark manual vive en
[`benches/tls_bench.rs`](../benches/tls_bench.rs). Mide negociación estructural,
rechazo por identidad y rechazo por suite obsoleta.

## Ejercicios

### Ejercicio 1: Validar identidad `[Nivel 1]`

Crea una cadena donde el certificado de hoja tenga sujeto
`api.jeresoft.test` y negocia con un cliente que pide ese mismo nombre.

**Entrada/Salida esperada:** la negociación devuelve `TlsVersion::V1_3`.

<details>
<summary>Pista</summary>
El nombre solicitado debe coincidir con el sujeto del certificado de hoja.
</details>

### Ejercicio 2: Negociar suite `[Nivel 2]`

Crea un cliente con dos suites y un servidor con una intersección. Verifica cuál
suite se elige.

**Entrada/Salida esperada:** se elige una suite común permitida.

<details>
<summary>Pista</summary>
La implementación recorre las suites del cliente y elige la primera que el
servidor también ofrece y la política permite.
</details>

### Ejercicio 3: Cadena incompleta `[Nivel 3]`

Intenta negociar con una cadena que solo contiene el certificado de hoja.

**Entrada/Salida esperada:** `TlsError::IncompleteCertificateChain`.

<details>
<summary>Pista</summary>
El modelo exige al menos hoja e intermediario.
</details>

### Ejercicio 4: Política de seguridad `[Nivel 4]`

Explica por qué una organización debería rechazar algoritmos obsoletos aunque
clientes antiguos todavía los soporten.

<details>
<summary>Pista</summary>
Compara compatibilidad contra riesgo de degradación y deuda operativa.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/tls_identity_check.rs`](../examples/soluciones/tls_identity_check.rs)
- [`examples/soluciones/tls_cipher_negotiation.rs`](../examples/soluciones/tls_cipher_negotiation.rs)
- [`examples/soluciones/tls_certificate_chain.rs`](../examples/soluciones/tls_certificate_chain.rs)

Para el nivel 4, una respuesta sana reconoce que compatibilidad sin política
puede reabrir riesgos ya conocidos. La seguridad se mantiene retirando opciones
débiles de forma explícita y medible.

## Referencias

- RFC 8446: *The Transport Layer Security (TLS) Protocol Version 1.3*.
- RFC 5246: *The Transport Layer Security (TLS) Protocol Version 1.2*.
- Ivan Ristic, *Bulletproof TLS and PKI*.
