---
titulo: "Clasificar archivos por extensión"
mundo: 22-sistemas-y-ffi
tema: archivos
dificultad: facil
---

Escribe `contar_por_extension(rutas: &[&str]) -> BTreeMap<String, usize>` que cuente cuántos archivos hay de cada extensión, usando `std::path::Path`.

- La extensión se cuenta en minúsculas (`"FOTO.JPG"` cuenta como `"jpg"`).
- Los archivos sin extensión se cuentan como `"(ninguna)"`.
- Un nombre que empieza por punto, como `.gitignore`, **no** tiene extensión (así lo decide `Path`).
