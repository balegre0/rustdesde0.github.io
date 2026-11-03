---
titulo: "Contar en compilación"
mundo: 19-macros
tema: macros
dificultad: media
---

Escribe la macro `cuenta!` que reciba expresiones separadas por comas (cero o más, coma final opcional) y se expanda a un **`usize` constante** con cuántas hay. Las expresiones **no deben evaluarse**: solo se cuentan.

```rust
const N: usize = cuenta!(a, b + 1, f(x));   // 3
let arr = [0u8; cuenta!(1, 2)];              // array de 2
cuenta!()                                    // 0
```

<details><summary>Pista 1</summary>

Recursión: `cuenta!(x, resto...)` = `1usize + cuenta!(resto...)`.
</details>

<details><summary>Pista 2</summary>

Para que la expresión no se evalúe ni tenga que existir, no la pongas en la plantilla. Ni siquiera hace falta que `a` o `f` existan: si nunca aparecen en el código expandido, el compilador no los busca.
</details>
