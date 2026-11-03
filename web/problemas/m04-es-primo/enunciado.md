---
titulo: "¿Es primo?"
mundo: 04-expresiones-y-control
tema: bucles
dificultad: media
---

Escribe el cuerpo de `es_primo`: devuelve `true` si `n` es primo (solo divisible por 1 y por sí mismo) y `false` si no. Recuerda: 0 y 1 **no** son primos.

## Ejemplos

| Entrada | Salida |
|---|---|
| `es_primo(2)` | `true` |
| `es_primo(91)` | `false` (7 × 13) |
| `es_primo(97)` | `true` |

<details><summary>Pista 1</summary>

Trata primero el caso `n < 2`. Después prueba divisores `d` desde 2.
</details>

<details><summary>Pista 2</summary>

Basta con probar mientras `d * d <= n`: si `n` tuviera un divisor mayor que su raíz, tendría también uno menor. Con `loop` o `while` y un `break` (o un bloque etiquetado) puedes salir en cuanto encuentres uno. Ojo: `d * d` con `u32` puede desbordar si `n` es enorme; usa `u64` para `d`.
</details>
