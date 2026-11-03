---
titulo: "Contar valores de un tipo"
mundo: 24-teoria-y-diseno
tema: tipos
dificultad: media
---

Representa tipos sencillos con este `enum` y calcula cuántos valores tiene cada uno (su **cardinalidad**):

```rust
enum Tipo {
    Unidad,                         // ()          → 1
    Bool,                           // bool        → 2
    U8,                             // u8          → 256
    Vacio,                          // Infallible  → 0
    Tupla(Vec<Tipo>),               // (A, B, …)   → producto
    Enum(Vec<Tipo>),                // variantes   → suma
    Opcion(Box<Tipo>),              // Option<T>   → |T| + 1
    Array(Box<Tipo>, u32),          // [T; n]      → |T| elevado a n
}
```

Escribe `cardinalidad(t: &Tipo) -> u128`. Una tupla vacía tiene 1 valor (es `()`) y un enum sin variantes, 0.
