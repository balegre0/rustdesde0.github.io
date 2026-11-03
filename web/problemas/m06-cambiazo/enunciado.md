---
titulo: "El cambiazo"
mundo: 06-ownership
tema: ownership
dificultad: dificil
---

Escribe `retirar_largos`: recibe un préstamo exclusivo de un `Vec<String>` y un máximo `max` (en caracteres). Cada texto con **más** de `max` caracteres se saca del vector y se deja en su lugar un `String` vacío. Devuelve, en orden, los textos sacados.

La longitud del vector **no cambia**, y no se puede clonar.

## Ejemplos

| Vector antes | `max` | Devuelve | Vector después |
|---|---|---|---|
| `["hola", "sí", "camión"]` | 3 | `["hola", "camión"]` | `["", "sí", ""]` |
| `["año", "años"]` | 3 | `["años"]` | `["año", ""]` |
| `["a"]` | 5 | `[]` | `["a"]` |

Ojo: `"año".len()` es 4 (bytes), pero tiene 3 caracteres.

<details>
<summary>Pista 1</summary>

Para contar caracteres: `s.chars().count()`.
</details>

<details>
<summary>Pista 2</summary>

`std::mem::take(&mut v[i])` saca el `String` de la posición `i` y deja en su lugar `String::new()`: el vector nunca tiene un hueco, así que no hay E0507.
</details>
