---
titulo: "Un literal de HashMap"
mundo: 19-macros
tema: macros
dificultad: facil
---

Escribe la macro `mapa!` que construya un `std::collections::HashMap` a partir de parejas `clave => valor` separadas por comas. Debe aceptar el mapa vacío y una coma final.

```rust
let m = mapa! { "uno" => 1, "dos" => 2, };
assert_eq!(m["dos"], 2);
let vacio: HashMap<i32, i32> = mapa! {};
```

Usa rutas absolutas (`::std::collections::HashMap`) para que la macro funcione aunque quien la llame no haya importado `HashMap`.

<details><summary>Pista</summary>

`$($k:expr => $v:expr),* $(,)?` en el patrón y un bloque `{{ let mut m = ...; $( m.insert($k, $v); )* m }}` en la plantilla.
</details>
