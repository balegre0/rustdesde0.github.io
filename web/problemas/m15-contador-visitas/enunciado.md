---
titulo: "Contador de visitas con Cell"
mundo: 15-punteros-inteligentes
tema: mutabilidad-interior
dificultad: facil
---

Una `Pagina` tiene un título y cuenta cuántas veces se ha visitado. El método `visitar` recibe solo `&self` (un préstamo **compartido**), porque muchas partes del programa pueden tener la página a la vez. Aun así, tiene que poder sumar una visita.

```rust
use std::cell::Cell;

struct Pagina {
    titulo: String,
    visitas: Cell<u32>,
}
```

Escribe:

- `Pagina::nueva(titulo: &str) -> Pagina`, con 0 visitas.
- `visitar(&self) -> u32`: suma una visita y devuelve el total **después** de sumar.
- `reiniciar(&self) -> u32`: pone las visitas a 0 y devuelve las que había.

```text
let p = Pagina::nueva("inicio");
p.visitar()   → 1
p.visitar()   → 2
p.reiniciar() → 2
p.visitar()   → 1
```

<details><summary>Pista</summary>

`Cell` tiene `get()` (copia del valor), `set(v)` y `replace(v)` (pone `v` y devuelve el valor anterior). Todos funcionan con `&self`.

</details>
