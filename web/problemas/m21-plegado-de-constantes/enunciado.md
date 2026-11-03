---
titulo: "Plegado de constantes"
mundo: 21-el-compilador-por-dentro
tema: compilador
dificultad: media
---

Una de las primeras optimizaciones de un compilador es el **plegado de constantes** (*constant folding*): calcular al compilar lo que no depende de nada desconocido.

Con este árbol de expresiones:

```rust
enum Expr { Num(i64), Var(String), Suma(Box<Expr>, Box<Expr>), Mul(Box<Expr>, Box<Expr>) }
```

escribe `plegar(e: Expr) -> Expr` que:

1. pliegue recursivamente los hijos;
2. si los dos hijos son `Num`, sustituya la operación por su resultado (con `wrapping_add`/`wrapping_mul`);
3. aplique las identidades `x + 0 = x`, `0 + x = x`, `x * 1 = x`, `1 * x = x` y `x * 0 = 0`, `0 * x = 0`.

Y `mostrar(e: &Expr) -> String`, con paréntesis en cada operación: `(x + (2 * y))`.
