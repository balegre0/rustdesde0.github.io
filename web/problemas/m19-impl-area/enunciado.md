---
titulo: "Un impl para cada figura"
mundo: 19-macros
tema: macros
dificultad: media
---

Tienes el trait `Area` (ya escrito en `inicio.rs`, no lo cambies) y varias figuras. Escribe la macro `impl_area!` para que esto:

```rust
impl_area! {
    Cuadrado, c => c.lado * c.lado;
    Rectangulo, r => r.ancho * r.alto;
}
```

genere un `impl Area for Cuadrado { fn area(&self) -> f64 { ... } }` y otro para `Rectangulo`, donde el nombre que va antes de `=>` (aquí `c` o `r`) sea **el nombre que la fórmula usa para `self`**. Debe aceptar un `;` final opcional.

<details><summary>Pista 1</summary>

Patrón: `$($tipo:ty, $s:ident => $formula:expr);* $(;)?`.
</details>

<details><summary>Pista 2 (higiene)</summary>

Dentro de `area`, escribe `let $s = self; $formula`. Como `$s` y la fórmula vienen los dos del sitio de la llamada, tienen el mismo contexto de higiene y se «ven». Una variable que escribieras tú en la plantilla (por ejemplo `let c = self;`) **no** la vería la fórmula.
</details>
