---
titulo: "Invertir en el sitio"
mundo: 07-prestamos
tema: prestamos
dificultad: media
---

Escribe `invertir`: recibe un préstamo exclusivo de un slice de `String` y le da la vuelta **en el sitio**, sin crear otro vector y sin clonar.

## Ejemplos

| Antes | Después |
|---|---|
| `["a", "b", "c", "d"]` | `["d", "c", "b", "a"]` |
| `["x", "y", "z"]` | `["z", "y", "x"]` |
| `[]` | `[]` |

<details>
<summary>Pista 1</summary>

`std::mem::swap(&mut v[i], &mut v[j])` no compila (E0499): el borrow checker no sabe que `i` y `j` son distintos. Los slices tienen el método `v.swap(i, j)`, que lo hace por ti de forma segura.
</details>

<details>
<summary>Pista 2</summary>

Basta con recorrer la primera mitad: intercambia `i` con `v.len() - 1 - i` para `i` en `0..v.len() / 2`.
</details>
