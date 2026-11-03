---
titulo: "Dar la vuelta a una tupla"
mundo: 03-valores-y-tipos
tema: tuplas
dificultad: facil
---

Escribe `intercambiar(par: (i32, char)) -> (char, i32)`: recibe una tupla con un número y un carácter y devuelve **otra** tupla con los dos elementos en el orden contrario.

| Entrada | Salida |
|---|---|
| `(7, 'a')` | `('a', 7)` |
| `(-3, 'ñ')` | `('ñ', -3)` |
| `(0, '🦀')` | `('🦀', 0)` |

Fíjate en que el **tipo** de la tupla también cambia: entra `(i32, char)` y sale `(char, i32)`.

<details>
<summary>Pista 1</summary>

Puedes leer cada parte con `par.0` y `par.1`.
</details>

<details>
<summary>Pista 2</summary>

O desestructurar: `let (numero, letra) = par;`. La última línea de la función, sin `;`, es lo que devuelve: `(letra, numero)`.
</details>
