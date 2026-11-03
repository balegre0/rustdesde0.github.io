---
titulo: "Un mapa de estaciones con índices"
mundo: 15-punteros-inteligentes
tema: rc-refcell
dificultad: media
---

Un mapa de metro es un **grafo**: estaciones unidas por vías, con ciclos por todas partes. Con `Rc<RefCell<…>>` sería un nido de contadores y posibles fugas. Lo haremos con una **arena**: un `Vec` es el dueño de todas las estaciones y cada vía se guarda como un índice `usize`.

```rust
struct Estacion {
    nombre: String,
    vias: Vec<usize>,
}

struct Mapa {
    estaciones: Vec<Estacion>,
}
```

Escribe estos métodos de `Mapa`:

- `nueva_estacion(&mut self, nombre: &str) -> usize`: añade una estación sin vías y devuelve su índice.
- `unir(&mut self, a: usize, b: usize)`: añade una vía en **los dos sentidos** (`a → b` y `b → a`).
- `alcanzables(&self, desde: usize) -> Vec<String>`: los nombres de todas las estaciones a las que se puede llegar desde `desde` (incluida ella misma), **ordenados alfabéticamente**.

```text
Sol —— Ópera —— Callao        Retiro (sin vías)
alcanzables(Sol)    → ["Callao", "Sol", "Ópera"]
alcanzables(Retiro) → ["Retiro"]
```

(Ojo: `sort()` ordena por bytes, así que «Ópera» va después de «Sol».)

<details><summary>Pista 1</summary>

Recorre el grafo con una lista de pendientes (`Vec<usize>`) y un `Vec<bool>` de visitadas del mismo tamaño que `estaciones`. Mientras queden pendientes: saca una, y si no estaba visitada, márcala y añade sus vías a pendientes.

</details>

<details><summary>Pista 2</summary>

Al final, recoge los nombres de las visitadas con `.clone()` y llama a `sort()` sobre el `Vec<String>`.

</details>
