---
titulo: "Fibonacci sin explotar"
mundo: 05-funciones-y-la-pila
tema: recursion
dificultad: dificil
---

Escribe `fibonacci`: `fib(0) = 0`, `fib(1) = 1` y `fib(n) = fib(n − 1) + fib(n − 2)`. Las pruebas llegan hasta `fib(90)`.

La versión recursiva directa hace **miles de millones** de llamadas para `n = 90` y no terminaría a tiempo. Encuentra una forma de calcularlo en unos 90 pasos.

## Ejemplos

| Entrada | Salida |
|---|---|
| `fibonacci(0)` | `0` |
| `fibonacci(10)` | `55` |
| `fibonacci(90)` | `2880067194370816120` |

<details><summary>Pista 1</summary>

Solo necesitas recordar los **dos últimos** valores. Empieza con `let mut a = 0; let mut b = 1;`.
</details>

<details><summary>Pista 2</summary>

Repite `n` veces: el nuevo par es `(b, a + b)`. Al final, `a` es la respuesta.
</details>
