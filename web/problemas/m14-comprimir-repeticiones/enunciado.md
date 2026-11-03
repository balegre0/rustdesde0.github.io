---
titulo: "Comprimir repeticiones"
mundo: 14-closures-e-iteradores
tema: iteradores
dificultad: dificil
---

Escribe `comprimir(texto: &str) -> Vec<(char, usize)>`: agrupa los caracteres **consecutivos** iguales y devuelve cada carácter con cuántas veces se repite seguido.

```text
comprimir("aaabcc") → [('a', 3), ('b', 1), ('c', 2)]
comprimir("abab")   → [('a', 1), ('b', 1), ('a', 1), ('b', 1)]
comprimir("ññn")    → [('ñ', 2), ('n', 1)]
comprimir("")       → []
```

Trabaja con caracteres (`chars()`), no con bytes.

<details><summary>Pista 1</summary>

`texto.chars().peekable()` te da un iterador con `peek()`: mirar el siguiente carácter sin sacarlo.

</details>

<details><summary>Pista 2</summary>

Con `while let Some(c) = it.next()`: empieza a contar en 1 y, mientras `it.peek() == Some(&c)`, saca otro y suma. Después guarda `(c, cuenta)`. Hay un atajo: `it.next_if_eq(&c)` saca el siguiente solo si es igual a `c`.

</details>
