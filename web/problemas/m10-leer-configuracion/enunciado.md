---
titulo: "Leer un archivo de configuración"
mundo: 10-errores
tema: errores
dificultad: media
---

Un archivo de configuración tiene líneas `clave = valor`. Las líneas vacías y las que empiezan por `#` se ignoran. Escribe:

```rust
fn leer(texto: &str) -> Result<Config, ErrorConfig>
```

`Config` tiene `nombre: String`, `puerto: u16` y `depurar: bool`. Errores, con el número de línea (desde 1):

- una línea sin `=` → `ErrorConfig::Formato(linea)`;
- una clave desconocida → `ErrorConfig::ClaveDesconocida(linea, clave)`;
- un `puerto` que no es un `u16` o un `depurar` que no es `true`/`false` → `ErrorConfig::Valor(linea, clave)`;
- si al final falta alguna de las tres claves → `ErrorConfig::Falta(clave)` (comprobando en el orden `nombre`, `puerto`, `depurar`).

Quita los espacios de alrededor de la clave y el valor (`trim`).

<details>
<summary>Pista</summary>

Guarda cada valor en un `Option` mientras lees. Al final, `nombre.ok_or(ErrorConfig::Falta("nombre"))?`. Para el número de línea, `texto.lines().enumerate()`.
</details>
