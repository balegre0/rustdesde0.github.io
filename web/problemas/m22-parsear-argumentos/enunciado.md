---
titulo: "Leer opciones de la línea de órdenes"
mundo: 22-sistemas-y-ffi
tema: procesos
dificultad: media
---

Un programa se llama como `copiar [-v] [--max N] ORIGEN DESTINO`. Escribe `parsear(args: &[&str]) -> Result<Opciones, String>` (los `args` **no** incluyen el nombre del programa):

- `-v` activa `verboso`.
- `--max N` fija `max` (un `u32`); si falta `N` o no es un número: `Err("--max necesita un número")`.
- Cualquier otro argumento que empiece por `-`: `Err("opción desconocida: X")`.
- Los demás son posicionales: tiene que haber exactamente dos (origen y destino); si no, `Err("uso: copiar [-v] [--max N] ORIGEN DESTINO")`.
- Si no se da `--max`, vale `None`.
