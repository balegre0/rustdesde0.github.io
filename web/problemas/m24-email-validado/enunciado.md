---
titulo: "Validar una vez"
mundo: 24-teoria-y-diseno
tema: diseno
dificultad: media
---

«*Parse, don't validate*»: en vez de comprobar un texto cada vez que lo usas, conviértelo **una vez** en un tipo que garantice la propiedad.

Completa `Email`, un newtype con el campo **privado** `texto: String` (dentro del módulo `correo`):

- `Email::parsear(s: &str) -> Result<Email, &'static str>`. Reglas, en este orden:
  1. sin espacios en los extremos (usa `trim`) y no vacío → si no, `Err("vacío")`;
  2. exactamente una `@` → si no, `Err("hace falta una @")`;
  3. algo antes de la `@` → si no, `Err("falta el usuario")`;
  4. después de la `@`, un dominio con al menos un `.` que no esté ni al principio ni al final → si no, `Err("dominio no válido")`.

  Guarda el texto en minúsculas.
- `usuario(&self) -> &str` y `dominio(&self) -> &str`: pueden partir el texto **sin comprobar nada**, porque el tipo ya lo garantiza.
