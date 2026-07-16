# Modelo de capas, IP y enrutamiento

> **Curso:** rust-networking · **Capítulo:** 01 · **Prerrequisitos:** Rust
> básico, estructuras de datos simples y representación binaria elemental
> **Código:** [`src/layers.rs`](../src/layers.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Una red moderna parece una sola cosa hasta que falla. Entonces aparecen capas,
direcciones, rutas, TTL, máscaras, puertas de enlace y paquetes que se pierden. Este
capítulo enseña el mapa mental mínimo para no depurar redes a ciegas: los datos
se encapsulan por capas, IP entrega paquetes con mejor esfuerzo y el
enrutamiento elige una salida según el prefijo más específico.

El modelo del crate no implementa una pila de red real. Representa las
invariantes que un ingeniero necesita para leer trazas, entender documentación y
razonar sobre comunicación entre sistemas.

## Motivación

Cuando una aplicación no puede llamar a una API, el problema puede estar en la
URL, DNS, TLS, TCP, IP, una ruta, un firewall o el servidor. Si todo se llama
"la red", no hay forma ordenada de investigar. Las capas existen para separar
responsabilidades.

El primer paso es entender qué pregunta responde cada capa:

```text
Aplicación: ¿qué quiere decir este mensaje?
Transporte: ¿cómo conversan dos procesos?
Internet: ¿cómo llega un paquete a otra red?
Enlace: ¿cómo cruza el siguiente salto físico o local?
```

IP no promete entrega perfecta. Promete intentar mover paquetes hacia su
destino. El TTL evita que un paquete viva para siempre si las rutas forman un
ciclo. La tabla de rutas decide por dónde mandar un paquete usando el prefijo
más específico que coincida con el destino.

## Teoría

### Historia

El modelo OSI separó la comunicación en siete capas para enseñar y estandarizar
responsabilidades. En la práctica, muchas herramientas usan el modelo TCP/IP:
aplicación, transporte, internet y enlace. Este curso usa ese modelo pragmático
porque conecta directo con Rust, sockets, HTTP, DNS, TLS y operación diaria.

### Fundamentos

Encapsular significa envolver datos con metadatos de una capa. Una solicitud
HTTP se vuelve parte de un segmento TCP, que se vuelve parte de un paquete IP,
que se vuelve parte de una trama de enlace.

Una dirección IPv4 tiene cuatro octetos. Un CIDR combina una dirección base y
un prefijo. `192.168.10.0/24` representa todas las direcciones cuyo primer
veinticuatro bits coinciden con `192.168.10`.

Una tabla de rutas contiene destinos posibles. La regla central es:

```text
si varias rutas coinciden, gana la ruta con el prefijo más específico
```

`10.20.0.0/16` gana contra `10.0.0.0/8` para el destino `10.20.30.40`.
Si no hay coincidencia, el sistema no sabe por dónde enviar el paquete.

El TTL se decrementa en cada salto. Cuando llega a cero, el paquete se descarta.
Esto vuelve finitos los ciclos de enrutamiento.

### Casos de uso

Este modelo ayuda a:

- leer salidas de `ping`, `traceroute`, `ip route` y tablas de ruteo;
- entender subredes privadas y rutas por defecto;
- diagnosticar por qué un servicio local no llega a otro;
- separar fallas de DNS, transporte, TLS y aplicación;
- diseñar redes internas de nube, VPC y túneles;
- preparar los capítulos de TCP, UDP, DNS, TLS, HTTP y QUIC.

### Ventajas y limitaciones

Ventajas:

- Da un mapa mental estable para depurar.
- Permite razonar sin depender de una herramienta específica.
- Conecta fundamentos con problemas reales de operación.
- Hace explícita la entrega de mejor esfuerzo de IP.

Limitaciones:

- El modelo educativo no serializa paquetes reales.
- No cubre IPv6 todavía.
- No modela ARP, NAT, firewall ni MTU en detalle.
- No sustituye herramientas del sistema operativo.
- No explica internos del núcleo del sistema operativo; eso vive en
  `rust-operating-systems`.

### Comparación con alternativas

El modelo OSI es más detallado y útil para separar responsabilidades
conceptuales, pero puede sentirse lejano al depurar aplicaciones modernas.

El modelo TCP/IP es más compacto y más cercano a la práctica diaria. Por eso el
curso lo usa como columna principal.

Depurar por herramienta sin modelo mental puede funcionar para casos simples,
pero se vuelve frágil cuando una falla cruza DNS, TLS, rutas o balanceadores.

## Diagramas

El diagrama principal vive en
[`diagrams/01-layers-ip-routing.mmd`](../diagrams/01-layers-ip-routing.mmd).
Muestra encapsulación, decremento de TTL y selección de ruta por prefijo más
específico.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `NetworkLayer::as_str` | O(1) | O(1) | O(1) | O(1) |
| `EncapsulatedFrame::wrap` | O(1) | O(1) | O(1) | O(1) adicional |
| `EncapsulatedFrame::describe_path` | O(l) | O(l) | O(l) | O(l) |
| `EncapsulatedFrame::decrement_ttl` | O(1) | O(1) | O(1) | O(1) |
| `Ipv4Address::to_u32` | O(1) | O(1) | O(1) | O(1) |
| `Ipv4Cidr::contains` | O(1) | O(1) | O(1) | O(1) |
| `RoutingTable::select_route` | O(r) | O(r) | O(r) | O(1) |

`l` es el número de capas registradas y `r` el número de rutas.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura debería permitir elegir un destino,
ver qué ruta gana, disminuir TTL salto por salto y observar cuándo un paquete se
descarta.

## Implementación

La implementación define:

- `NetworkLayer`: capas Application, Transport, Internet y Link;
- `EncapsulatedFrame`: carga útil, capas envolventes y TTL;
- `Ipv4Address`: dirección IPv4 de cuatro octetos;
- `Ipv4Cidr`: rango IPv4 con prefijo;
- `Route`: destino, interfaz y siguiente salto opcional;
- `RoutingTable`: selección por prefijo más específico;
- `RouteDecision`: resultado explícito de enrutamiento;
- `TtlError`: descarte por TTL agotado.

La API evita sockets reales. Esto permite probar invariantes sin depender del
sistema operativo ni de una red externa.

## Pruebas

Las pruebas cubren:

- orden de encapsulación de capas;
- capa más externa;
- descripción de camino;
- pertenencia IPv4 a CIDR;
- selección de ruta por prefijo más específico;
- error cuando TTL llega a cero.

## Benchmarks

El benchmark manual vive en
[`benches/layers_bench.rs`](../benches/layers_bench.rs). Mide selección de ruta
en tablas pequeñas y medianas, además del costo de comprobación CIDR.

El objetivo no es comparar contra el núcleo del sistema operativo. El objetivo
es observar cómo una búsqueda lineal crece con el número de rutas y por qué las
pilas reales usan estructuras especializadas.

## Ejercicios

### Ejercicio 1: Clasificar un paquete `[Nivel 1]`

Crea un `EncapsulatedFrame` con capas Application, Transport e Internet.

**Entrada/Salida esperada:** `outermost_layer()` devuelve `Internet`.

<details>
<summary>Pista</summary>
El frame guarda las capas en orden de adentro hacia afuera.
</details>

### Ejercicio 2: Coincidencia de subred `[Nivel 2]`

Crea `10.20.0.0/16` y verifica si `10.20.30.40` pertenece a la subred.

**Entrada/Salida esperada:** `contains` devuelve `true`.

<details>
<summary>Pista</summary>
El prefijo compara los primeros bits de la dirección.
</details>

### Ejercicio 3: Selección de ruta `[Nivel 3]`

Crea una tabla con ruta por defecto, `10.0.0.0/8` y `10.20.0.0/16`.
Consulta `10.20.30.40`.

**Entrada/Salida esperada:** gana la interfaz de `10.20.0.0/16`.

<details>
<summary>Pista</summary>
La ruta con el prefijo más largo es la más específica.
</details>

### Ejercicio 4: Diagnóstico de red `[Nivel 4]`

Explica cómo investigarías una falla donde una app no puede llamar a
`https://api.example.com`, pero `ping 8.8.8.8` sí responde.

<details>
<summary>Pista</summary>
Separa IP/rutas, DNS, TCP, TLS y HTTP antes de culpar a "la red".
</details>

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/layers_classify_packet.rs`](../examples/soluciones/layers_classify_packet.rs)
- [`examples/soluciones/layers_subnet_match.rs`](../examples/soluciones/layers_subnet_match.rs)
- [`examples/soluciones/layers_route_selection.rs`](../examples/soluciones/layers_route_selection.rs)

Para el nivel 4, una respuesta sana empieza por confirmar resolución DNS,
conectividad TCP al puerto 443, validación TLS y respuesta HTTP. Que `ping`
funcione solo prueba una parte baja del camino.

## Referencias

- Andrew S. Tanenbaum y David J. Wetherall, *Computer Networks*.
- James F. Kurose y Keith W. Ross, *Computer Networking: A Top-Down Approach*.
- RFC 791: *Internet Protocol*.
- RFC 1812: *Requirements for IP Version 4 Routers*.
