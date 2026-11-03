---
titulo: "Exclamar"
mundo: 06-ownership
tema: ownership
dificultad: facil
---

Escribe `exclamar`: recibe un `String` **por valor** (se queda con la propiedad), le añade `n` signos `!` al final y lo **devuelve**.

No hace falta crear un `String` nuevo: el que recibes ya es tuyo, así que puedes modificarlo (declara el parámetro como `mut`).

## Ejemplos

| Llamada | Resultado |
|---|---|
| `exclamar(String::from("hola"), 2)` | `"hola!!"` |
| `exclamar(String::from("año"), 1)` | `"año!"` |
| `exclamar(String::new(), 3)` | `"!!!"` |

<details>
<summary>Pista 1</summary>

Escribe la firma como `fn exclamar(mut s: String, n: usize) -> String`. El `mut` es del parámetro: como eres dueño, puedes decidir modificarlo.
</details>

<details>
<summary>Pista 2</summary>

`s.push('!')` dentro de un `for _ in 0..n`, y al final la expresión `s` (sin punto y coma) devuelve la propiedad.
</details>
