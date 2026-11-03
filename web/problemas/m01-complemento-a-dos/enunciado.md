---
titulo: "Leer complemento a dos"
mundo: 01-la-maquina
tema: bits
dificultad: media
---

Un byte se puede leer como **sin signo** (0 a 255) o **con signo** en complemento a dos (−128 a 127). Escribe `con_signo(bits: &str) -> i8` que reciba exactamente 8 caracteres `0`/`1` y devuelva el número con signo, **calculándolo a mano** (sin `as i8` ni `i8::from_str_radix`).

Regla: el bit de más peso vale **−128** en lugar de +128; los demás valen lo de siempre (64, 32, …, 1).

Ejemplos: `"00000101"` → 5, `"11111111"` → −1, `"10000000"` → −128.

<details>
<summary>Pista</summary>

Recorre los caracteres con su posición. El peso del bit de la posición `i` (empezando por la izquierda en 0) es `1 << (7 - i)`, y si `i == 0`, réstalo en vez de sumarlo. Acumula en un `i32` y conviértelo al final.
</details>
