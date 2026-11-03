---
titulo: "La más larga"
mundo: 08-lifetimes
tema: lifetimes
dificultad: facil
---

Completa `mas_larga`: devuelve la vista con más bytes de las dos; si empatan, la primera.

La firma ya está escrita. Léela así: «para alguna región `'a` en la que `a` y `b` sean válidas, el resultado también lo es». Por eso puedes devolver cualquiera de las dos.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `mas_larga("sol", "luna")` | `"luna"` |
| `mas_larga("mar", "río")` | `"río"` («río» tiene 4 bytes) |
| `mas_larga("ab", "cd")` | `"ab"` |

<details>
<summary>Pista</summary>

Un `if a.len() >= b.len() { a } else { b }` como expresión final.
</details>
