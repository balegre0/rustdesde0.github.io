---
titulo: "Unir nombres"
mundo: 06-ownership
tema: ownership
dificultad: facil
---

Escribe `unir`: recibe un `Vec<String>` **por valor** y devuelve un único `String` con todos los nombres separados por `", "`.

Como el vector es tuyo, puedes recorrerlo con `for nombre in v`, que va entregando cada `String` con su propiedad.

## Ejemplos

| Entrada | Resultado |
|---|---|
| `["Ana", "Luis", "Eva"]` | `"Ana, Luis, Eva"` |
| `["Íñigo"]` | `"Íñigo"` |
| `["", "x"]` | `", x"` |
| `[]` | `""` |

<details>
<summary>Pista 1</summary>

Empieza con `let mut resultado = String::new();` y una variable `let mut primero = true;`. Antes de añadir cada nombre, si no es el primero, añade `", "` con `push_str`. (No mires si `resultado` está vacío: un nombre puede ser `""`.)
</details>

<details>
<summary>Pista 2</summary>

`push_str` recibe un `&str`: pásale `&nombre`. Al terminar cada vuelta, `nombre` se libera.
</details>
