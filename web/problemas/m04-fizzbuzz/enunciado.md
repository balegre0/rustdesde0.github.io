---
titulo: "FizzBuzz"
mundo: 04-expresiones-y-control
tema: match
dificultad: facil
---

Escribe el cuerpo de `fizzbuzz`: devuelve `"FizzBuzz"` si `n` es múltiplo de 3 y de 5, `"Fizz"` si solo lo es de 3, `"Buzz"` si solo lo es de 5, y el número como texto en otro caso.

## Ejemplos

| Entrada | Salida |
|---|---|
| `fizzbuzz(9)` | `"Fizz"` |
| `fizzbuzz(10)` | `"Buzz"` |
| `fizzbuzz(30)` | `"FizzBuzz"` |
| `fizzbuzz(7)` | `"7"` |

<details><summary>Pista 1</summary>

Haz `match` sobre la tupla `(n % 3, n % 5)`. El patrón `(0, 0)` va primero.
</details>

<details><summary>Pista 2</summary>

Para devolver un texto propio: `String::from("Fizz")`. Para convertir el número: `n.to_string()`.
</details>
