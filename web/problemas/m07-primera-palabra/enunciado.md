---
titulo: "La primera palabra"
mundo: 07-prestamos
tema: prestamos
dificultad: facil
---

Escribe `primera_palabra`: recibe un texto prestado y devuelve **una vista** (`&str`) de su primera palabra, es decir, todo lo que hay antes del primer espacio. Si no hay espacios, la palabra es el texto entero.

No crees ningún `String`: devuelve un trozo del texto original.

## Ejemplos

| Entrada | Resultado |
|---|---|
| `"hola mundo"` | `"hola"` |
| `"España"` | `"España"` |
| `" empieza con espacio"` | `""` |

<details>
<summary>Pista 1</summary>

`s.find(' ')` devuelve `Some(posición en bytes)` o `None`. Con un `match` decides qué trozo devolver.
</details>

<details>
<summary>Pista 2</summary>

`&s[..i]` es la vista desde el principio hasta el byte `i` (sin incluirlo). Un espacio es un solo byte, así que nunca cortas un carácter por la mitad.
</details>
