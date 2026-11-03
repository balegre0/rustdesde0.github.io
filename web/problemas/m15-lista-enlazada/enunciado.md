---
titulo: "Longitud y máximo de una lista con Box"
mundo: 15-punteros-inteligentes
tema: box
dificultad: facil
---

Tienes una lista recursiva hecha con `Box`:

```rust
enum Lista {
    Nodo(i32, Box<Lista>),
    Vacia,
}
```

Escribe dos funciones:

- `longitud(l: &Lista) -> usize`: cuántos nodos tiene la lista.
- `maximo(l: &Lista) -> Option<i32>`: el mayor número de la lista, o `None` si está vacía.

```text
1 → 5 → 3 → fin      longitud = 3, maximo = Some(5)
fin                  longitud = 0, maximo = None
-4 → -9 → fin        longitud = 2, maximo = Some(-4)
```

<details><summary>Pista 1</summary>

Usa `match l { Lista::Nodo(valor, resto) => ..., Lista::Vacia => ... }`. Como `l` es una referencia, `valor` será `&i32` y `resto` será `&Box<Lista>`; gracias a la coerción de desreferencia puedes pasar `resto` directamente a una función que pide `&Lista`.

</details>

<details><summary>Pista 2</summary>

Para `maximo`, calcula primero el máximo del resto. Si es `None`, el máximo es tu valor; si es `Some(m)`, compara tu valor con `m`.

</details>
