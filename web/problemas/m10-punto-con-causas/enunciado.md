---
titulo: "Un punto con causas"
mundo: 10-errores
tema: errores
dificultad: dificil
---

Hay que leer puntos con el formato `x,y` (dos `i32`). El tipo de error ya está declarado:

```rust
enum ErrorPunto {
    SinComa,
    Coordenada { eje: char, causa: ParseIntError },
}
```

Tienes que escribir **cuatro** cosas:

1. `impl fmt::Display for ErrorPunto`: `SinComa` → `falta la coma`; `Coordenada { eje, .. }` → `coordenada x inválida` (o `y`). **No** incluyas la causa en el texto.
2. `impl Error for ErrorPunto` con `source()`: devuelve la `causa` en `Coordenada` y `None` en `SinComa`.
3. `leer_punto(texto: &str) -> Result<Punto, ErrorPunto>`: separa por la **primera** coma, recorta espacios de cada parte y convierte cada una. Si falla la `x`, el error lleva `eje: 'x'`; si falla la `y`, `eje: 'y'`.
4. `distancia2(a: &str, b: &str) -> Result<i64, Box<dyn Error>>`: lee los dos puntos con `?` y devuelve el cuadrado de su distancia `(x2-x1)² + (y2-y1)²` como `i64`.

<details>
<summary>Pista 1</summary>

`.map_err(|causa| ErrorPunto::Coordenada { eje: 'x', causa })?` construye el error con la causa dentro.
</details>

<details>
<summary>Pista 2</summary>

En `distancia2`, `?` convierte `ErrorPunto` en `Box<dyn Error>` solo porque implementa `Error`. Convierte las coordenadas a `i64` antes de restar para no desbordar.
</details>
