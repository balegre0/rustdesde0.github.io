---
titulo: "Calculadora que no explota"
mundo: 09-structs-enums-y-patrones
tema: option
dificultad: media
---

Escribe `aplicar(op, a, b)`, que aplica una operación del enum `Op` a dos `i32` y devuelve `Option<i32>`:

- `Some(resultado)` si la operación es válida.
- `None` si se divide entre 0 **o** si el resultado no cabe en un `i32` (desbordamiento).

Nada de pánicos: con cualquier entrada, la función tiene que devolver algo.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `aplicar(Op::Sumar, 2, 3)` | `Some(5)` |
| `aplicar(Op::Dividir, 7, 2)` | `Some(3)` |
| `aplicar(Op::Dividir, 1, 0)` | `None` |
| `aplicar(Op::Multiplicar, i32::MAX, 2)` | `None` |

<details>
<summary>Pista 1</summary>

Los enteros tienen métodos `checked_add`, `checked_sub`, `checked_mul` y `checked_div` que ya devuelven `Option`: `None` si hay desbordamiento (o división entre 0).
</details>

<details>
<summary>Pista 2</summary>

Ojo con `i32::MIN / -1`: el resultado (2 147 483 648) no cabe en un `i32`. `checked_div` también lo detecta.
</details>
