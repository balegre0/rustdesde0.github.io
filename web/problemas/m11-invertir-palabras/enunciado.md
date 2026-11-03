---
titulo: "Invertir cada palabra"
mundo: 11-colecciones-y-texto
tema: texto
dificultad: facil
---

Escribe `invertir_palabras(frase: &str) -> String` que invierta **cada palabra** pero mantenga el orden de las palabras.

- Las palabras están separadas por un solo espacio.
- Invierte por **caracteres**, no por bytes: `"año"` → `"oña"`.

Ejemplo: `"hola mundo"` → `"aloh odnum"`.

<details>
<summary>Pista</summary>

`p.chars().rev().collect::<String>()` invierte una palabra. Para juntar con espacios, recoge en un `Vec<String>` y usa `join(" ")`.
</details>
