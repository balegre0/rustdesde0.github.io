---
titulo: "Un módulo de temperaturas"
mundo: 12-modulos-crates-y-cargo
tema: modulos
dificultad: facil
---

Las pruebas usan `temperatura::Celsius` y `temperatura::conversion::a_fahrenheit`. Completa el módulo `temperatura` para que:

- `Celsius` sea un struct público con un campo **privado** `grados: f64`.
- `Celsius::nueva(g: f64) -> Option<Celsius>` devuelva `None` si `g` está por debajo del cero absoluto (−273.15) y `Some` en otro caso.
- `Celsius::grados(&self) -> f64` devuelva el valor.
- El submódulo público `conversion` tenga `a_fahrenheit(c: &Celsius) -> f64` (F = C × 9/5 + 32).

<details>
<summary>Pista</summary>

Dentro de `conversion`, el struct es `super::Celsius`. Como `conversion` es hijo de `temperatura`, puede leer el campo privado `grados` directamente.
</details>
