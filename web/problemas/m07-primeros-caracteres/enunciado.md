---
titulo: "Los primeros n caracteres"
mundo: 07-prestamos
tema: prestamos
dificultad: dificil
---

Escribe `primeros`: devuelve una vista con los primeros `n` **caracteres** de `s` (no bytes). Si `s` tiene menos de `n` caracteres, devuelve `s` entero.

Cuidado: `&s[..n]` cuenta bytes, y si cae en mitad de una «ñ» el programa hace pánico (*byte index is not a char boundary*).

## Ejemplos

| Llamada | Resultado |
|---|---|
| `primeros("año nuevo", 3)` | `"año"` |
| `primeros("ñu", 1)` | `"ñ"` |
| `primeros("hola", 10)` | `"hola"` |
| `primeros("hola", 0)` | `""` |

<details>
<summary>Pista 1</summary>

`s.char_indices()` recorre pares `(posición en bytes, carácter)`. La posición del carácter número `n` (contando desde 0) es justo donde hay que cortar.
</details>

<details>
<summary>Pista 2</summary>

Con `s.char_indices().nth(n)` obtienes `Some((byte, c))` o `None` si no hay tantos caracteres.
</details>
