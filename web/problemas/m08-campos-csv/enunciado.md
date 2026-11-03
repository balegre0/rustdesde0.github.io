---
titulo: "Un cursor de campos"
mundo: 08-lifetimes
tema: lifetimes
dificultad: dificil
---

`Campos<'a>` recorre una línea separada por comas y entrega los campos uno a uno con `siguiente`, que devuelve `Some(campo)` o `None` cuando ya no quedan.

El campo `resto` es lo que falta por leer: `Some(texto)` mientras queden campos (aunque sea uno vacío) y `None` al terminar. Así `"a,,b"` da `"a"`, `""`, `"b"`, y `""` da un único campo vacío.

La firma devuelve `Option<&'a str>`: los campos son vistas de la **línea**, no del cursor. Si escribieras `Option<&str>`, la elisión los ataría a `&mut self` y no podrías guardar un campo mientras pides el siguiente (E0499). Las pruebas guardan varios a la vez.

## Ejemplo

`Campos { resto: Some("año,mes,,día") }` → `Some("año")`, `Some("mes")`, `Some("")`, `Some("día")`, `None`.

<details>
<summary>Pista 1</summary>

Saca el texto con `let texto = self.resto?;` (si es `None`, la función devuelve `None`). Después busca `texto.find(',')`.
</details>

<details>
<summary>Pista 2</summary>

Si hay coma en `i`: guarda `self.resto = Some(&texto[i + 1..])` y devuelve `Some(&texto[..i])`. Si no hay: `self.resto = None` y devuelve `Some(texto)`.
</details>
