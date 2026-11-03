---
titulo: "Una calculadora polaca inversa en compilación"
mundo: 19-macros
tema: macros
dificultad: dificil
---

En **notación polaca inversa** (RPN) los operadores van detrás de sus operandos: `3 4 +` es `3 + 4`, y `3 4 + 2 *` es `(3 + 4) * 2`. Se evalúa con una pila: cada número se apila; cada operador saca los dos de arriba y apila el resultado.

Escribe la macro `rpn!` (un *tt-muncher*) que reciba números literales y los operadores `+`, `-` y `*`, y se expanda a la expresión aritmética correspondiente.

```rust
rpn!(3 4 +)          // 7
rpn!(3 4 + 2 *)      // 14
rpn!(10 2 3 * -)     // 10 - (2 * 3) = 4
```

En `a b -` el resultado es `a - b` (el de abajo menos el de arriba).

<details><summary>Pista 1</summary>

Guarda la pila en una regla interna: `(@ [cima, debajo, ...] tokens que quedan...)`.
</details>

<details><summary>Pista 2</summary>

Pon primero la regla final (`(@ [$x:expr])`), después una regla por operador (`(@ [$b:expr, $a:expr $(, $resto:expr)*] + $($t:tt)*)`) y después la de los números (`$n:literal`). La regla de entrada, la última.
</details>
