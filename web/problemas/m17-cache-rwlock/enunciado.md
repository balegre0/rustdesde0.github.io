---
titulo: "Una caché con RwLock"
mundo: 17-concurrencia
tema: mutex
dificultad: media
---

Una caché compartida entre hilos guarda resultados ya calculados. Muchos hilos leen y pocos escriben, así que usa un `RwLock<HashMap<u64, u64>>`.

Escribe `Cache::obtener(&self, n: u64, calcular: impl Fn(u64) -> u64) -> u64`:

1. Con un **candado de lectura**, mira si `n` ya está. Si está, devuelve el valor.
2. Si no, **suelta** el de lectura, toma uno de **escritura**, calcula, guarda y devuelve.

Lleva la cuenta de cuántas veces se llamó a `calcular` en el campo atómico `calculos`.

<details>
<summary>Pista</summary>

Mete la lectura en su propio bloque `{ … }` (o usa la expresión directamente) para que el guard de lectura se suelte antes de pedir `write()`: si no, el hilo se bloquearía esperándose a sí mismo. Con escritura, usa `entry(n).or_insert_with(|| …)`: otro hilo pudo calcularlo entre medias.
</details>
