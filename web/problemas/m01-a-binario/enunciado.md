---
titulo: "Escribir en binario"
mundo: 01-la-maquina
tema: bits
dificultad: facil
---

Escribe `a_binario(n: u8) -> String` que devuelva los **8 bits** de `n`, del más significativo al menos, sin usar `format!("{:b}")`.

Ejemplos: `5` → `"00000101"`, `255` → `"11111111"`, `128` → `"10000000"`.

<details>
<summary>Pista</summary>

El bit `i` de `n` vale `(n >> i) & 1`. Recorre `i` de 7 a 0 con `(0..8).rev()`.
</details>
