---
titulo: "La media de cinco notas"
mundo: 03-valores-y-tipos
tema: conversiones
dificultad: dificil
---

Cinco jueces puntúan una actuación con un `u8` cada uno (de 0 a 255). Escribe `media(notas: [u8; 5]) -> f64`, que devuelve la media **con decimales**.

| Entrada | Salida |
|---|---|
| `media([10, 10, 10, 10, 10])` | `10.0` |
| `media([1, 2, 3, 4, 5])` | `3.0` |
| `media([0, 0, 0, 0, 1])` | `0.2` |
| `media([255, 255, 255, 255, 255])` | `255.0` |

Dos trampas:

- La media de `[0, 0, 0, 0, 1]` es `0.2`, no `0`: no hagas la división con enteros.
- La suma de cinco notas de 255 es 1275, que **no cabe en un `u8`**. Si sumas en `u8`, en debug tendrás un pánico.

<details>
<summary>Pista 1</summary>

Recorre el array con `for nota in notas { ... }` y acumula en una variable `let mut suma: u32 = 0;`.
</details>

<details>
<summary>Pista 2</summary>

Para sumar un `u8` a un `u32` hay que convertirlo: `suma += nota as u32;` (o `u32::from(nota)`). Al final, `suma as f64 / 5.0`.
</details>
