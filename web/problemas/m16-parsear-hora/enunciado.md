---
titulo: "Una hora sin trampas"
mundo: 16-pruebas-y-herramientas
tema: fuzzing
dificultad: media
---

En la lección de fuzzing, un generador aleatorio descubrió que una función de horas aceptaba `"+0005:5"`. Escribe tú la versión **estricta**:

`parsear_hora(texto: &str) -> Option<(u8, u8)>` devuelve `Some((horas, minutos))` solo si `texto` tiene **exactamente** la forma `HH:MM`:

- 5 caracteres: dos dígitos, `:`, dos dígitos (nada de signos, espacios ni dígitos de más o de menos);
- horas de `00` a `23` y minutos de `00` a `59`.

En cualquier otro caso, `None`. Nunca debe entrar en pánico, reciba lo que reciba (las pruebas incluyen textos con «ñ» y emojis).

## Ejemplos

| Llamada | Resultado |
|---|---|
| `parsear_hora("09:05")` | `Some((9, 5))` |
| `parsear_hora("23:59")` | `Some((23, 59))` |
| `parsear_hora("24:00")` | `None` |
| `parsear_hora("+0005:5")` | `None` |
| `parsear_hora("9:05")` | `None` |

<details>
<summary>Pista 1</summary>

Trabaja con los **bytes**: `let b = texto.as_bytes();`. Si `b.len() != 5`, ya puedes devolver `None`. Un byte `c` es un dígito si `c.is_ascii_digit()`.
</details>

<details>
<summary>Pista 2</summary>

Cuidado con cortar el texto con `&texto[0..2]` antes de comprobar nada: con `"1ñ:0"` (5 bytes, porque la `ñ` ocupa 2) cortar `[0..2]` parte la `ñ` por la mitad, y cortar en un sitio que no es frontera de carácter entra en pánico. Con los bytes no hay ese problema. El valor de un byte dígito es `b - b'0'`.
</details>
