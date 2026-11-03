---
titulo: "Líneas que contienen"
mundo: 08-lifetimes
tema: lifetimes
dificultad: media
---

Escribe `lineas_con`: devuelve, en orden, las líneas de `texto` que contienen `patron`, como vistas del texto.

Fíjate en la firma: `patron` **no** lleva `'a`. Las líneas devueltas salen de `texto`, no de `patron`, así que el patrón puede morir antes que el resultado. Si pusieras `&'a str` también en `patron`, la función compilaría igual… pero las pruebas no (allí el patrón es un `String` que se libera antes de usar las líneas).

## Ejemplo

Con `texto = "uno\ndos\ntres\ndoce"` y `patron = "do"` → `["dos", "doce"]`.

<details>
<summary>Pista 1</summary>

`texto.lines()` recorre las líneas como `&'a str`; `linea.contains(patron)` dice si contiene el patrón.
</details>

<details>
<summary>Pista 2</summary>

Empieza con `let mut v = Vec::new();` y haz `push` de cada línea que cumpla.
</details>
