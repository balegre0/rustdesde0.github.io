---
titulo: "Quitar espacios dobles sin reservar de más"
mundo: 15-punteros-inteligentes
tema: cow
dificultad: media
---

Escribe `normaliza(texto: &str) -> Cow<'_, str>`, que sustituya cada grupo de **dos o más espacios seguidos** por **un solo espacio**.

La mayoría de los textos ya vienen bien. En ese caso **no** debes reservar memoria: devuelve `Cow::Borrowed(texto)`. Solo si hay algo que cambiar, construye un `String` nuevo y devuelve `Cow::Owned`.

```text
"hola mundo"        → "hola mundo"      (Borrowed)
"hola   mundo"      → "hola mundo"      (Owned)
"  año  nuevo "     → " año nuevo "     (Owned)
""                  → ""                (Borrowed)
```

Las pruebas comprueban el texto **y** la variante.

<details><summary>Pista 1</summary>

`texto.contains("  ")` te dice si hay al menos dos espacios seguidos. Si no los hay, ya puedes devolver el préstamo.

</details>

<details><summary>Pista 2</summary>

Para construir el resultado, recorre `texto.chars()` y recuerda si el último carácter que añadiste era un espacio: si lo era y el actual también, sáltalo.

</details>
