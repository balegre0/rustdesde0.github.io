---
titulo: "Torres de Hanói"
mundo: 05-funciones-y-la-pila
tema: recursion
dificultad: dificil
---

En las Torres de Hanói hay que mover `n` discos de una varilla a otra, de uno en uno y sin poner nunca un disco sobre otro más pequeño. La estrategia recursiva para mover `n` discos es:

1. mover los `n − 1` de arriba a la varilla auxiliar,
2. mover el disco grande a su destino,
3. mover los `n − 1` encima del grande.

Escribe `movimientos_hanoi(n)` de forma **recursiva**, siguiendo esa estrategia: devuelve cuántos movimientos hacen falta.

## Ejemplos

| Entrada | Salida |
|---|---|
| `movimientos_hanoi(0)` | `0` |
| `movimientos_hanoi(3)` | `7` |
| `movimientos_hanoi(10)` | `1023` |

<details><summary>Pista 1</summary>

Los pasos 1 y 3 cuestan lo mismo: `movimientos_hanoi(n - 1)` cada uno. El paso 2 es un movimiento.
</details>

<details><summary>Pista 2</summary>

`2 * movimientos_hanoi(n - 1) + 1`, con caso base `n == 0 → 0`. Calcula la llamada recursiva una sola vez y multiplica.
</details>
