---
titulo: "Pasos de Collatz"
mundo: 04-expresiones-y-control
tema: while
dificultad: media
---

La regla de Collatz: si `n` es par, pasa a `n / 2`; si es impar, a `3n + 1`. Escribe el cuerpo de `pasos_collatz`: devuelve cuántos pasos hacen falta para llegar a 1 empezando en `n` (con `n ≥ 1`).

## Ejemplos

| Entrada | Salida | Por qué |
|---|---|---|
| `pasos_collatz(1)` | `0` | ya es 1 |
| `pasos_collatz(6)` | `8` | 6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 |
| `pasos_collatz(27)` | `111` | |

<details><summary>Pista 1</summary>

`while n != 1`, con un contador de pasos.
</details>

<details><summary>Pista 2</summary>

El parámetro `n` no es `mut`: cópialo primero (`let mut x = n;`).
</details>
