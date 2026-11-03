---
titulo: "Repartir en dos equipos"
mundo: 06-ownership
tema: ownership
dificultad: media
---

Escribe `repartir`: recibe un `Vec<String>` por valor y reparte sus nombres en dos vectores nuevos: los de posición par (0, 2, 4…) al primero y los de posición impar al segundo, **sin clonar** ningún texto. Devuelve los dos como una tupla.

## Ejemplos

| Entrada | Resultado |
|---|---|
| `["a", "b", "c", "d", "e"]` | `(["a", "c", "e"], ["b", "d"])` |
| `["solo"]` | `(["solo"], [])` |
| `[]` | `([], [])` |

<details>
<summary>Pista 1</summary>

Si recorres `for nombre in v` cada `nombre` es un `String` tuyo y puedes moverlo con `push`. Lleva tú la cuenta de la posición con una variable `i`.
</details>

<details>
<summary>Pista 2</summary>

Un número es par si `i % 2 == 0`.
</details>
