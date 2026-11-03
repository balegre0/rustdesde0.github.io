---
titulo: "Un grep pequeño"
mundo: 22-sistemas-y-ffi
tema: io
dificultad: media
---

Escribe `buscar(consulta: &str, entrada: impl BufRead, ignorar_mayusculas: bool) -> Vec<(usize, String)>` que devuelva las líneas que contienen `consulta`, con su número de línea (empezando en 1).

- Si `ignorar_mayusculas` es `true`, compara en minúsculas.
- Las líneas se devuelven tal como estaban (sin el salto de línea).
- Si una línea no se puede leer, ignórala.

Al recibir cualquier `BufRead`, la misma función sirve para la entrada estándar, para un archivo o, como en las pruebas, para un texto en memoria (`Cursor`).
