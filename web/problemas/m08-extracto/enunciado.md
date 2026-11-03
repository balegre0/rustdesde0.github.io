---
titulo: "Un struct que mira un texto"
mundo: 08-lifetimes
tema: lifetimes
dificultad: media
---

`Extracto<'a>` guarda una vista de un texto que vive en otro sitio. Completa sus métodos:

- `primera_frase`: el trozo hasta el primer `'.'` (sin incluirlo); si no hay punto, el texto entero.
- `palabras`: cuántas palabras tiene (separadas por espacios en blanco).

`primera_frase` devuelve `&'a str` (la lifetime del **texto**), no un préstamo de `self`: así la frase puede seguir usándose aunque el `Extracto` desaparezca. Las pruebas lo comprueban.

## Ejemplo

`Extracto { texto: "Era una vez. Fin." }`: `primera_frase()` → `"Era una vez"`, `palabras()` → 4.

<details>
<summary>Pista 1</summary>

`self.texto.find('.')` y luego `&self.texto[..i]`. Como `self.texto` es `&'a str` (y `&str` es `Copy`), el trozo también es `&'a str`.
</details>

<details>
<summary>Pista 2</summary>

`self.texto.split_whitespace().count()`.
</details>
