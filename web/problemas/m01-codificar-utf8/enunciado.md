---
titulo: "Codificar en UTF-8 a mano"
mundo: 01-la-maquina
tema: texto
dificultad: dificil
---

Escribe `a_utf8(c: char) -> Vec<u8>` que devuelva los bytes UTF-8 de un carácter, **sin** usar `encode_utf8` ni `to_string`. Las reglas, según el valor `v = c as u32`:

| Rango de `v` | Bytes | Forma (las `x` son los bits de `v`) |
|---|---|---|
| `0` a `0x7F` | 1 | `0xxxxxxx` |
| `0x80` a `0x7FF` | 2 | `110xxxxx 10xxxxxx` |
| `0x800` a `0xFFFF` | 3 | `1110xxxx 10xxxxxx 10xxxxxx` |
| `0x10000` a `0x10FFFF` | 4 | `11110xxx 10xxxxxx 10xxxxxx 10xxxxxx` |

Ejemplos: `'a'` → `[0x61]`, `'ñ'` (U+00F1) → `[0xC3, 0xB1]`, `'€'` (U+20AC) → `[0xE2, 0x82, 0xAC]`.

<details>
<summary>Pista</summary>

Los 6 bits bajos de `v` son `v & 0x3F`; para ponerles la marca `10`, `0x80 | (v & 0x3F)`. El primer byte de 2 bytes es `0xC0 | (v >> 6)`; el de 3, `0xE0 | (v >> 12)`, y luego `0x80 | ((v >> 6) & 0x3F)`…
</details>
