---
titulo: "Un lexer de verdad"
mundo: 21-el-compilador-por-dentro
tema: compilador
dificultad: media
---

Escribe `tokenizar(texto: &str) -> Result<Vec<Token>, String>`, el primer paso de cualquier compilador.

Reglas:

- Los espacios (`' '`, `'\n'`, `'\t'`) separan tokens y se descartan.
- Una secuencia de dígitos es `Token::Num(valor)` (cabe en `u64`).
- Una secuencia que empieza por letra o `_` y sigue con letras, dígitos o `_` es un identificador; si es `let` o `fn`, es `Token::Clave(String)`, si no `Token::Ident(String)`.
- `+ - * / = ( ) ;` son `Token::Simbolo(char)`.
- Cualquier otro carácter devuelve `Err(format!("carácter inesperado '{c}' en {pos}"))`, donde `pos` es el índice en **bytes** del carácter.

Ejemplo: `"let x1 = 42;"` → `[Clave("let"), Ident("x1"), Simbolo('='), Num(42), Simbolo(';')]`.

<details><summary>Pista 1</summary>

Usa `texto.char_indices().peekable()`: te da cada carácter con su posición en bytes y te deja mirar el siguiente sin consumirlo.

</details>

<details><summary>Pista 2</summary>

Para un identificador, acumula en un `String` mientras `peek()` sea alfanumérico o `_`, y al final compara con `"let"` y `"fn"`.

</details>
