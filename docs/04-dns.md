# DNS

> **Curso:** rust-networking · **Capítulo:** 04 · **Prerrequisitos:** modelo de
> capas, UDP, TCP básico y lectura de direcciones IP
> **Código:** [`src/dns.rs`](../src/dns.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

DNS convierte nombres humanos en datos que una aplicación puede usar para
conectarse: direcciones IPv4, direcciones IPv6, alias, servidores de correo y
texto asociado a un dominio. Sin DNS, muchas aplicaciones tendrían que depender
de direcciones fijas o archivos locales difíciles de operar.

Este capítulo modela una zona autoritativa, un resolvedor con caché, registros
comunes y cadenas CNAME. El objetivo no es implementar un resolvedor real de
internet, sino entender las invariantes que explican por qué un nombre resuelve,
por qué deja de resolver o por qué una respuesta vieja puede seguir viva hasta
que expire su TTL.

## Motivación

Cuando una aplicación no puede llamar a `api.jeresoft.test`, el problema puede
estar en muchas capas: el nombre no existe, el alias apunta a otro nombre, la
caché conserva una respuesta anterior, el TTL todavía no expira, la zona no
tiene el tipo de registro solicitado o la red falla después de resolver.

DNS existe para separar identidad legible de ubicación operativa:

```text
el código usa nombres; la infraestructura decide a dónde apuntan
```

## Teoría

### Historia

Antes de DNS, los sistemas podían usar archivos locales con listas de nombres y
direcciones. Ese enfoque se vuelve frágil al crecer: cada cambio requiere
distribuir archivos y cada equipo puede quedar con una versión distinta.

DNS introdujo una base distribuida y jerárquica. Las zonas autoritativas
declaran respuestas, los resolvedores consultan y cachean, y el TTL define por
cuánto tiempo una respuesta puede reutilizarse.

### Fundamentos

Una zona contiene registros. En este capítulo se modelan:

- `A`: nombre hacia dirección IPv4;
- `AAAA`: nombre hacia dirección IPv6;
- `CNAME`: alias hacia otro nombre;
- `MX`: servidor de correo;
- `TXT`: texto asociado a un dominio.

Un resolvedor pregunta por un nombre y un tipo. Si encuentra el registro,
devuelve una resolución. Si encuentra un `CNAME`, sigue el alias hasta llegar al
nombre canónico. Para evitar ciclos o cadenas absurdas, el resolvedor tiene un
límite de saltos.

El TTL es una promesa temporal: una respuesta puede guardarse en caché hasta que
expire. En el crate, el reloj se inyecta como un número entero para que las
pruebas sean deterministas.

`NXDOMAIN` significa que el nombre no existe en la zona consultada. Es distinto
de "el nombre existe, pero no tiene el tipo de registro solicitado".

### Resolución recursiva y autoridad

En DNS real, un resolvedor recursivo puede consultar raíz, TLD y servidores
autoritativos hasta encontrar una respuesta. Este crate no implementa esa red de
consultas. Modela la parte conceptual: una zona autoritativa responde y un
resolvedor cachea resultados.

La autoridad importa porque no todas las respuestas tienen el mismo peso. Una
respuesta autoritativa viene de quien administra la zona. Una respuesta en caché
puede ser correcta, pero solo hasta que expire.

### Casos de uso

Este modelo ayuda a:

- diagnosticar por qué una API no resuelve;
- entender cambios de infraestructura que tardan por TTL;
- separar errores de DNS de errores TCP, TLS o HTTP;
- razonar sobre alias CNAME;
- preparar capítulos de HTTP, HTTPS, SMTP, gRPC y QUIC;
- entender por qué MX y TXT son parte de operación cotidiana.

### Ventajas y limitaciones

Ventajas:

- Desacopla nombres legibles de direcciones concretas.
- Permite cambiar infraestructura sin cambiar código.
- Reduce consultas repetidas mediante caché.
- Hace explícito el tiempo de vida de una respuesta.
- Permite alias y registros especializados.

Limitaciones:

- Una caché puede conservar respuestas antiguas hasta que expire el TTL.
- CNAME agrega saltos y puede complicar diagnóstico.
- DNS por sí mismo no cifra ni autentica todas las respuestas en este modelo.
- El crate no implementa DNSSEC, transporte real ni resolución recursiva de
  internet.
- Un nombre que resuelve no garantiza que TCP, TLS o HTTP funcionen.

### Comparación con archivos hosts y descubrimiento de servicios

Un archivo `hosts` es simple y local. Sirve para pruebas o excepciones, pero no
escala bien porque cada máquina puede tener una versión distinta.

DNS centraliza autoridad por zona y permite caché controlada por TTL. Es mejor
para infraestructura compartida y cambios operativos.

El descubrimiento de servicios suele resolver instancias dinámicas dentro de un
entorno específico. Puede usar DNS por debajo, pero agrega salud, balanceo,
metadatos o integración con orquestadores.

## Diagramas

El diagrama principal vive en
[`diagrams/04-dns.mmd`](../diagrams/04-dns.mmd). Muestra consulta, caché, zona,
seguimiento de CNAME y expiración por TTL.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `DomainName::new` | O(n) | O(n) | O(n) | O(n) |
| `Zone::add_record` | O(log z) | O(log z) | O(log z) | O(1) adicional |
| `Resolver::resolve` con caché vigente | O(log c) | O(log c) | O(log c) | O(1) |
| `Resolver::resolve` con zona | O(r) | O(c * r log z) | O(c * r log z) | O(r) |
| `Resolution::records` | O(1) | O(1) | O(1) | O(1) |

`n` es la longitud del nombre, `z` el número de nombres en la zona, `c` el
número de saltos CNAME y `r` el número de registros por nombre.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura debería permitir consultar un nombre,
ver si la respuesta sale de caché, seguir CNAME y avanzar el reloj hasta que el
TTL expire.

## Implementación

La implementación define:

- `DomainName`: nombre normalizado;
- `RecordType`: tipos `A`, `AAAA`, `CNAME`, `MX` y `TXT`;
- `DnsRecord`: registro con nombre, tipo, valor y TTL;
- `Zone`: conjunto autoritativo de registros;
- `Resolver`: resolvedor con caché y límite CNAME;
- `Resolution`: resultado con nombre canónico, registros y origen de caché;
- `DnsError`: nombre inválido, NXDOMAIN, tipo ausente y límite CNAME.

El resolvedor recibe `now` como valor numérico. Esto evita depender del reloj del
sistema y permite probar expiración de caché de forma repetible.

## Pruebas

Las pruebas cubren:

- resolución de registro A;
- seguimiento de cadena CNAME;
- uso de caché hasta que expira el TTL;
- error NXDOMAIN para nombres inexistentes.

## Benchmarks

El benchmark manual vive en
[`benches/dns_bench.rs`](../benches/dns_bench.rs). Mide resolución con caché
fría, caché caliente y resolución de alias CNAME.

El objetivo es observar la diferencia entre consultar la zona y reutilizar una
respuesta vigente en caché.

## Ejercicios

### Ejercicio 1: Registro A `[Nivel 1]`

Crea una zona con `api.jeresoft.test` apuntando a `203.0.113.10` y resuelve el
registro A.

**Entrada/Salida esperada:** la resolución contiene un registro A con esa
dirección.

<details>
<summary>Pista</summary>
Usa `DnsRecord::a` y después `Resolver::resolve`.
</details>

### Ejercicio 2: CNAME hacia A `[Nivel 2]`

Crea `api.jeresoft.test` como alias de `origin.jeresoft.test`; después agrega un
registro A para `origin.jeresoft.test`.

**Entrada/Salida esperada:** el nombre canónico es `origin.jeresoft.test`.

<details>
<summary>Pista</summary>
El resolvedor sigue CNAME cuando no encuentra el tipo solicitado en el nombre
original.
</details>

### Ejercicio 3: Caché y TTL `[Nivel 3]`

Resuelve un nombre con TTL 10 en el instante 100. Vuelve a resolver en 109 y en
111.

**Entrada/Salida esperada:** en 109 la respuesta viene de caché; en 111 se vuelve
a consultar la zona.

<details>
<summary>Pista</summary>
El reloj es un parámetro de `resolve`, no una llamada al sistema operativo.
</details>

### Ejercicio 4: Diagnóstico de API `[Nivel 4]`

Explica cómo investigarías una falla donde `api.example.com` no responde, pero
el servicio sí funciona cuando llamas directo a su dirección IP.

<details>
<summary>Pista</summary>
Separa NXDOMAIN, registro incorrecto, CNAME roto, TTL viejo y fallas posteriores
en TCP/TLS/HTTP.
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/dns_a_record.rs`](../examples/soluciones/dns_a_record.rs)
- [`examples/soluciones/dns_cname_resolution.rs`](../examples/soluciones/dns_cname_resolution.rs)
- [`examples/soluciones/dns_ttl_cache.rs`](../examples/soluciones/dns_ttl_cache.rs)

Para el nivel 4, una respuesta sana empieza por consultar el tipo de registro
correcto, revisar alias, verificar TTL de caché y solo después subir a transporte
y aplicación.

## Referencias

- Andrew S. Tanenbaum y David J. Wetherall, *Computer Networks*.
- James F. Kurose y Keith W. Ross, *Computer Networking: A Top-Down Approach*.
- RFC 1034: *Domain Names - Concepts and Facilities*.
- RFC 1035: *Domain Names - Implementation and Specification*.
