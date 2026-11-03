---
titulo: "Dividir en dos"
mundo: 08-lifetimes
tema: lifetimes
dificultad: media
---

Escribe `dividir`: corta `s` en la **primera** aparición de `sep` y devuelve las dos partes (sin el separador) como vistas del texto original. Si `sep` no aparece, devuelve `(s, "")`.

Las dos vistas salen de `s`, así que la firma ata las dos salidas a la misma `'a`.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `dividir("clave=valor", '=')` | `("clave", "valor")` |
| `dividir("a=b=c", '=')` | `("a", "b=c")` |
| `dividir("sin", '=')` | `("sin", "")` |
| `dividir("año:2026", ':')` | `("año", "2026")` |

<details>
<summary>Pista 1</summary>

`s.find(sep)` da la posición en bytes del separador. La segunda parte empieza en `i + sep.len_utf8()` (un `char` puede ocupar más de un byte).
</details>

<details>
<summary>Pista 2</summary>

El literal `""` es `&'static str`, y un `'static` vale como cualquier `'a` más corta.
</details>
