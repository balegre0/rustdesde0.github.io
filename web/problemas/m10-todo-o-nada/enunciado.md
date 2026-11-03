---
titulo: "Todo o nada"
mundo: 10-errores
tema: errores
dificultad: facil
---

Escribe `sumar(textos: &[&str]) -> Result<i64, String>`, que suma todos los números de la lista.

- Si todos los textos son números enteros (`i64`, se admiten espacios alrededor), devuelve `Ok(suma)`.
- En cuanto uno no lo sea, devuelve `Err` con el texto **exacto** `«X» no es un número`, donde `X` es el texto original (sin recortar).
- Una lista vacía suma `0`.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `sumar(&["1", " 2 ", "3"])` | `Ok(6)` |
| `sumar(&["1", "dos", "tres"])` | `Err("«dos» no es un número")` |
| `sumar(&[])` | `Ok(0)` |

<details>
<summary>Pista 1</summary>

Un bucle `for` con `?` funciona: `let n: i64 = t.trim().parse().map_err(|_| format!("«{t}» no es un número"))?;`.
</details>

<details>
<summary>Pista 2</summary>

En una sola expresión: `.map(…)` que devuelve `Result`, luego `.sum::<Result<i64, String>>()`. Sí: `sum` también sabe parar en el primer error.
</details>
