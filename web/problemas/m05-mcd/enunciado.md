---
titulo: "Máximo común divisor"
mundo: 05-funciones-y-la-pila
tema: recursion
dificultad: media
---

Escribe `mcd`, el máximo común divisor de dos números, con el algoritmo de Euclides: `mcd(a, 0) = a` y `mcd(a, b) = mcd(b, a % b)`.

## Ejemplos

| Entrada | Salida |
|---|---|
| `mcd(48, 36)` | `12` |
| `mcd(17, 5)` | `1` |
| `mcd(0, 9)` | `9` |

<details><summary>Pista</summary>

Es una sola línea: `if b == 0 { a } else { mcd(b, a % b) }`. ¿Por qué termina? Porque `a % b` siempre es menor que `b`.
</details>
