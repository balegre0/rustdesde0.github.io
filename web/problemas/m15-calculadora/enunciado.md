---
titulo: "Una calculadora con árboles de Box"
mundo: 15-punteros-inteligentes
tema: box
dificultad: media
---

Una expresión aritmética es un árbol: `(2 + 3) * -4` tiene una multiplicación en la raíz, una suma a la izquierda y un cambio de signo a la derecha. Como cada operación contiene otras expresiones, el tipo es recursivo y necesita `Box`:

```rust
enum Expr {
    Num(i64),
    Suma(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}
```

Escribe:

- `evalua(e: &Expr) -> i64`: el resultado de la expresión.
- `cuenta_ops(e: &Expr) -> usize`: cuántas operaciones (`Suma`, `Mul`, `Neg`) contiene; los números no cuentan.

```text
(2 + 3) * -4        evalua = -20, cuenta_ops = 3
7                   evalua = 7,   cuenta_ops = 0
-(-(1 + 1))         evalua = 2,   cuenta_ops = 3
```

<details><summary>Pista</summary>

Un `match` con una rama por variante. En `Suma(a, b)`, `a` y `b` son `&Box<Expr>`: puedes llamar a `evalua(a)` directamente gracias a la coerción de desreferencia.

</details>
