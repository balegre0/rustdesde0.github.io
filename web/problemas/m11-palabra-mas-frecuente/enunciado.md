---
titulo: "La palabra más frecuente"
mundo: 11-colecciones-y-texto
tema: colecciones
dificultad: media
---

Escribe `mas_frecuente(texto: &str) -> Option<(String, usize)>` que devuelva la palabra que más veces aparece y cuántas veces.

- Las palabras se separan por espacios en blanco (`split_whitespace`).
- No distingas mayúsculas de minúsculas: compara en minúsculas (`to_lowercase`), y devuelve la palabra en minúsculas.
- Si hay empate, gana la que va **antes en orden alfabético**.
- Si el texto no tiene palabras, `None`.

<details>
<summary>Pista</summary>

Cuenta con un `HashMap<String, usize>` y `entry(…).or_insert(0)`. Para elegir, recorre el mapa y quédate con la de mayor cuenta; en empate, la menor palabra. Un `BTreeMap` te da las claves ya ordenadas.
</details>
