---
titulo: "La letra del DNI"
mundo: 16-pruebas-y-herramientas
tema: pruebas
dificultad: facil
---

La letra de un DNI español se calcula así: el número (8 cifras) módulo 23 da una posición en la tabla `TRWAGMYFPDXBNJZSQVHLCKE`.

Escribe:

- `letra(numero: u32) -> char`, y
- `validar(dni: &str) -> bool`: `true` si el texto tiene exactamente 8 cifras seguidas de la letra correcta (en mayúscula).

Este problema viene con **muchas pruebas**, incluidos casos límite: es lo que hace un buen conjunto de tests. Léelas antes de empezar.
