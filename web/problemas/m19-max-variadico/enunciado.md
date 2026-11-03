---
titulo: "El máximo de cualquier número de valores"
mundo: 19-macros
tema: macros
dificultad: facil
---

Escribe la macro `max_de!` que reciba **una o más** expresiones separadas por comas (con coma final opcional) y devuelva la mayor.

```rust
max_de!(7)            // 7
max_de!(3, 9, 4)      // 9
max_de!(-5, -2, -8,)  // -2
```

Cada argumento debe evaluarse **una sola vez** (las pruebas lo comprueban con una función que cuenta sus llamadas).

<details><summary>Pista 1</summary>

Usa recursión: el máximo de `a, resto...` es el mayor entre `a` y el máximo de `resto...`.
</details>

<details><summary>Pista 2</summary>

Para evaluar una vez, guarda los valores en variables dentro de un bloque `{{ let a = $a; ... }}`, o usa `std::cmp::max`, que es una función y recibe valores ya calculados.
</details>
