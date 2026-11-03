---
titulo: "Doblar y sumar"
mundo: 07-prestamos
tema: prestamos
dificultad: facil
---

Escribe dos funciones:

- `doblar`: recibe un préstamo **exclusivo** de un slice de enteros y multiplica por 2 cada elemento, en el sitio.
- `suma`: recibe un préstamo **compartido** de un slice y devuelve la suma.

Como reciben slices (`&[i32]` / `&mut [i32]`), sirven para vectores, arrays y trozos.

## Ejemplos

| Datos | Tras `doblar` | `suma` |
|---|---|---|
| `[1, 2, 3]` | `[2, 4, 6]` | 12 |
| `[-5]` | `[-10]` | -10 |
| `[]` | `[]` | 0 |

<details>
<summary>Pista 1</summary>

`for x in v` con `v: &mut [i32]` te da `x: &mut i32`. Para cambiar el número: `*x *= 2`.
</details>

<details>
<summary>Pista 2</summary>

En `suma`, `for x in v` te da `&i32`; `total += x` funciona (o `*x`).
</details>
