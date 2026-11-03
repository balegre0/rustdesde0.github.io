---
titulo: "Bytes y letras"
mundo: 03-valores-y-tipos
tema: texto
dificultad: media
---

Escribe `medir(texto: &str) -> (usize, usize)`, que devuelve una tupla con:

1. el número de **bytes** que ocupa el texto en UTF-8, y
2. el número de **caracteres** (`char`) que contiene.

| Entrada | Salida |
|---|---|
| `medir("hola")` | `(4, 4)` |
| `medir("año")` | `(4, 3)` |
| `medir("€")` | `(3, 1)` |
| `medir("")` | `(0, 0)` |

En los textos con ñ, tildes o símbolos, los dos números **no** coinciden.

<details>
<summary>Pista 1</summary>

Uno de los dos números te lo da `len()`. ¿Cuál?
</details>

<details>
<summary>Pista 2</summary>

`texto.len()` cuenta bytes; `texto.chars().count()` cuenta caracteres. Devuelve los dos en una tupla: `(texto.len(), texto.chars().count())`.
</details>
