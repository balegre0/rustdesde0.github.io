---
titulo: "Sacar el más largo"
mundo: 06-ownership
tema: ownership
dificultad: media
---

Escribe `sacar_mas_largo`: recibe un préstamo exclusivo de un `Vec<String>`, **quita** del vector el texto con más bytes (si hay empate, el que aparece primero) y te entrega su propiedad. Si el vector está vacío, devuelve `None`.

Sin `clone()`: el texto tiene que salir del vector, no copiarse.

## Ejemplos

| Vector antes | Devuelve | Vector después |
|---|---|---|
| `["sol", "luna", "mar"]` | `Some("luna")` | `["sol", "mar"]` |
| `["ab", "cd"]` | `Some("ab")` | `["cd"]` |
| `[]` | `None` | `[]` |

<details>
<summary>Pista 1</summary>

`let x = v[i];` no compila (E0507): no se puede mover fuera de un índice. Primero busca **la posición** del más largo y luego usa `v.remove(posicion)`, que quita el elemento y te lo entrega.
</details>

<details>
<summary>Pista 2</summary>

Recorre con `for i in 0..v.len()` y compara `v[i].len()` con la mejor longitud vista (usa `>`, no `>=`, para quedarte con el primero en un empate).
</details>
