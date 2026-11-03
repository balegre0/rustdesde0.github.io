---
titulo: "Factorial recursivo"
mundo: 05-funciones-y-la-pila
tema: recursion
dificultad: media
---

Escribe `factorial` de forma **recursiva**: `0! = 1` y `n! = n × (n − 1)!`. Las pruebas usan `n ≤ 20` (21! ya no cabe en un `u64`).

## Ejemplos

| Entrada | Salida |
|---|---|
| `factorial(0)` | `1` |
| `factorial(5)` | `120` |
| `factorial(20)` | `2432902008176640000` |

<details><summary>Pista</summary>

Caso base: `if n == 0 { 1 }`. Caso recursivo: `n * factorial(n - 1)`.
</details>
