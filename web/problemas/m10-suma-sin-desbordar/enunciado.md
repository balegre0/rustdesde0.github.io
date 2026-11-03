---
titulo: "Sumar sin desbordar"
mundo: 10-errores
tema: errores
dificultad: facil
---

Escribe `sumar_todo(textos: &[&str]) -> Result<u8, ErrorSuma>` que convierta cada texto en un `u8` y los sume.

- Si un texto no es un número válido → `Err(ErrorSuma::NoEsNumero(i))`, con su posición `i` (desde 0).
- Si la suma no cabe en un `u8` → `Err(ErrorSuma::Desborda)`.

Usa `?` para propagar los errores y `checked_add` para la suma.
