---
titulo: "Aplica n veces"
mundo: 14-closures-e-iteradores
tema: closures
dificultad: facil
---

Escribe `aplicar_n(f, n, x)`, que aplique la función `f` **`n` veces seguidas** empezando en `x`: `f(f(f(…f(x))))`. Si `n` es 0, devuelve `x` sin tocarlo.

`f` es cualquier cosa que se pueda llamar con un `i64` y devuelva un `i64`: un closure o una función.

```text
aplicar_n(|x| x * 2, 3, 1)   → 8
aplicar_n(|x| x + 10, 0, 5)  → 5
aplicar_n(|x| x - 1, 4, 0)   → -4
```

<details><summary>Pista 1</summary>

¿Qué bound necesitas? Vas a llamar a `f` muchas veces, pero sin modificar nada: `impl Fn(i64) -> i64` basta.

</details>

<details><summary>Pista 2</summary>

Un `for _ in 0..n` con una variable acumuladora funciona. Con iteradores: `(0..n).fold(x, |acc, _| f(acc))`.

</details>
