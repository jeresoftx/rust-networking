# gRPC

> **Curso:** rust-networking · **Capítulo:** 10 · **Prerrequisitos:** HTTP/2
> conceptual, contratos de APIs y serialización
> **Código:** [`src/grpc.rs`](../src/grpc.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

gRPC es una forma de diseñar comunicación entre servicios a partir de contratos.
En vez de tratar cada interacción como una ruta libre, el sistema define
servicios, métodos, mensajes, estados y modos de flujo.

Este capítulo no implementa Protocol Buffers ni un servidor real. Modela la
semántica que un ingeniero debe entender antes de usar herramientas de
producción: contrato, método, versión, modo de flujo, mensaje y estado.

## Motivación

Cuando muchos servicios internos crecen juntos, la ambigüedad se vuelve cara.
Una llamada necesita saber qué método existe, qué versión espera, si devuelve
un solo mensaje o varios, y cómo reporta errores sin depender de texto libre.

La idea central es:

```text
gRPC convierte una llamada remota en un contrato explícito de servicio y método.
```

## Teoría

### Historia

gRPC se popularizó como una alternativa práctica para comunicación interna entre
servicios. Se apoya normalmente en HTTP/2 y Protocol Buffers, aunque la idea
central no es el formato, sino el contrato: servicios con métodos tipados y
estados conocidos.

### Fundamentos

Un servicio agrupa métodos. Un método declara nombre, versión y modo de flujo.
Un mensaje transporta una representación serializada. Un estado gRPC indica si
la llamada terminó bien o por qué falló.

El modelo del crate usa:

- `GrpcService`: nombre, versión y métodos registrados;
- `GrpcMethod`: nombre, versión de contrato y modo de flujo;
- `GrpcMessage`: nombre lógico de tipo y bytes;
- `GrpcStatus`: código, nombre y mensaje;
- `StreamMode`: unario, flujo del servidor, flujo del cliente o bidireccional.

### HTTP/2

gRPC suele usar HTTP/2 porque permite multiplexar varias llamadas sobre una
conexión, transportar metadatos y sostener flujos. Este capítulo no implementa
HTTP/2; lo trata como el transporte que hace viable la semántica de llamadas y
flujos.

### Modos de flujo

Hay cuatro formas comunes:

- Unario: un mensaje entra y un mensaje sale.
- Flujo del servidor: un mensaje entra y varios salen.
- Flujo del cliente: varios mensajes entran y uno sale.
- Bidireccional: ambos lados pueden enviar varios mensajes.

Elegir modo cambia la ergonomía, la presión de memoria y la estrategia de
errores.

### Estados

gRPC define códigos de estado propios. `OK` no es lo mismo que un `200` HTTP:
pertenece a la semántica de la llamada. Este modelo incluye estados frecuentes:
`OK`, `INVALID_ARGUMENT`, `NOT_FOUND` y `UNAVAILABLE`.

### Compatibilidad

Los contratos evolucionan. Una versión incompatible puede romper clientes aunque
el transporte funcione. El modelo valida versión por método para enseñar que la
compatibilidad es parte del diseño, no una nota al final.

### Casos de uso

gRPC aparece en:

- comunicación entre microservicios;
- servicios internos con contratos estables;
- flujos de telemetría;
- APIs generadas a partir de esquemas;
- sistemas donde latencia y tipado pesan más que legibilidad manual.

### Ventajas y limitaciones

Ventajas:

- Contratos explícitos.
- Buen soporte para flujos.
- Códigos de estado conocidos.
- Integración natural con generación de clientes.

Limitaciones:

- Menos amigable para exploración manual que HTTP/JSON.
- Requiere disciplina de versiones.
- El ecosistema de herramientas puede ocultar detalles del transporte.
- Este crate no implementa serialización ni red real.

### Comparación con REST

REST y gRPC no son bandos. REST suele ser excelente para APIs públicas,
recursos navegables y herramientas humanas. gRPC suele brillar en comunicación
interna con contratos fuertes, generación de clientes y flujos.

La decisión sana depende del consumidor, la estabilidad del contrato, la
operación y la facilidad de diagnóstico.

## Diagramas

El diagrama principal vive en
[`diagrams/10-grpc.mmd`](../diagrams/10-grpc.mmd). Muestra servicio, método,
mensaje, modo de flujo y estado.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `GrpcService::add_method` | O(log m) | O(log m) | O(log m) | O(n) |
| `GrpcService::method` | O(log m) | O(log m) | O(log m) | O(k) |
| `ensure_compatible` | O(log m) | O(log m) | O(log m) | O(k) |
| `GrpcMessage::new` | O(t + p) | O(t + p) | O(t + p) | O(t + p) |

`m` es la cantidad de métodos, `n` el tamaño del método, `k` el nombre buscado,
`t` el nombre del tipo y `p` el tamaño de la carga útil.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría cambiar modos de flujo y
versiones para mostrar qué llamadas son compatibles.

## Implementación

La implementación define `GrpcMethod`, `GrpcService`, `GrpcMessage`,
`GrpcStatus`, `StreamMode` y `GrpcError`. El servicio registra métodos, rechaza
duplicados, busca por nombre y valida compatibilidad de versión.

## Pruebas

Las pruebas cubren:

- registro de servicio y método;
- mapeo de códigos de estado;
- cuatro modos de flujo;
- incompatibilidad de versión;
- mensaje con nombre de tipo y carga útil.

## Benchmarks

El benchmark manual vive en
[`benches/grpc_bench.rs`](../benches/grpc_bench.rs). Mide registro de contratos,
validación de compatibilidad y construcción de mensajes.

## Ejercicios

### Ejercicio 1: Contrato de método `[Nivel 1]`

Registra `GetLesson` en `academy.LessonService`.

**Entrada/Salida esperada:** el método debe poder recuperarse por nombre.

### Ejercicio 2: Estado gRPC `[Nivel 2]`

Crea un estado `NOT_FOUND` con mensaje `lección no encontrada`.

**Entrada/Salida esperada:** el código debe ser `5`.

### Ejercicio 3: Modo de flujo `[Nivel 3]`

Modela un método de flujo del servidor.

**Entrada/Salida esperada:** `has_server_stream` debe devolver `true`.

### Ejercicio 4: REST o gRPC `[Nivel 4]`

Elige REST o gRPC para un servicio interno de recomendaciones y justifica la
decisión con contrato, consumidores y operación.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/grpc_method_contract.rs`](../examples/soluciones/grpc_method_contract.rs)
- [`examples/soluciones/grpc_status_mapping.rs`](../examples/soluciones/grpc_status_mapping.rs)
- [`examples/soluciones/grpc_streaming_model.rs`](../examples/soluciones/grpc_streaming_model.rs)
