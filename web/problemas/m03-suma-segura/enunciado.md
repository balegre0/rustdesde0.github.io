---
titulo: "Una suma de bytes que no desborda"
mundo: 03-valores-y-tipos
tema: overflow
dificultad: facil
---

Escribe `suma_segura(a: u8, b: u8) -> Option<u8>`: si `a + b` cabe en un `u8` (0 a 255), devuelve `Some(suma)`; si no cabe, devuelve `None`. En ningún caso debe haber un pánico, ni en debug ni en release.

| Entrada | Salida |
|---|---|
| `suma_segura(100, 50)` | `Some(150)` |
| `suma_segura(200, 55)` | `Some(255)` |
| `suma_segura(200, 56)` | `None` |
| `suma_segura(255, 255)` | `None` |

<details>
<summary>Pista 1</summary>

`a + b` con valores que no caben da pánico en debug y da la vuelta en release. Ninguna de las dos cosas es lo que te piden.
</details>

<details>
<summary>Pista 2</summary>

Hay un método de los enteros que devuelve justo un `Option`: `a.checked_add(b)`.
</details>
