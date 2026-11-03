---
titulo: "Comprimir y descomprimir"
mundo: 16-pruebas-y-herramientas
tema: propiedades
dificultad: dificil
---

Vas a escribir un compresor muy sencillo, la **codificación por longitud de series** (*run-length encoding*): cada grupo de caracteres iguales seguidos se escribe como «cuántos» seguido de «cuál».

- `comprimir("aaabcc")` → `"3a1b2c"`
- `comprimir("ñññ")` → `"3ñ"`
- `comprimir("")` → `""`

Y su inversa, `descomprimir`, que devuelve `None` si el texto no tiene un formato válido (un carácter sin número delante, o un número sin carácter detrás):

- `descomprimir("3a1b2c")` → `Some("aaabcc")`
- `descomprimir("12x")` → `Some("xxxxxxxxxxxx")` (los números pueden tener varias cifras)
- `descomprimir("a")` → `None`; `descomprimir("3")` → `None`

Puedes suponer que los textos que se comprimen **no contienen dígitos**.

La prueba estrella es una **propiedad de ida y vuelta**: para cualquier texto `s` sin dígitos, `descomprimir(&comprimir(s))` debe ser `Some(s)`. Se comprueba con 1000 textos aleatorios con «ñ», tildes, espacios y emojis.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `comprimir("aaabcc")` | `"3a1b2c"` |
| `descomprimir("2ñ1a")` | `Some("ñña")` |
| `descomprimir("x3")` | `None` |
| `descomprimir("0a")` | `Some("")` (cero veces «a») |

<details>
<summary>Pista 1</summary>

Para `comprimir`, recorre `s.chars()` llevando el carácter actual y cuántas veces lo has visto. Cuando cambia, escribe la cuenta y el carácter (`push_str(&cuenta.to_string())` y `push(c)`). No olvides el último grupo.
</details>

<details>
<summary>Pista 2</summary>

Para `descomprimir`, acumula dígitos en un número (`n = n * 10 + d`) hasta encontrar un carácter que no sea dígito; entonces añade ese carácter `n` veces. Si llega un carácter sin dígitos antes, o el texto termina con dígitos pendientes, devuelve `None`. `c.to_digit(10)` te da `Some(d)` si `c` es un dígito.
</details>
