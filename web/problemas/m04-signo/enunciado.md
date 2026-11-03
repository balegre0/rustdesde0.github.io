---
titulo: "El signo de un número"
mundo: 04-expresiones-y-control
tema: if
dificultad: facil
---

Escribe el cuerpo de `signo`: debe devolver `1` si `n` es positivo, `-1` si es negativo y `0` si es cero.

> Las funciones las estudiarás en el mundo 05. Aquí solo tienes que rellenar el cuerpo: el valor que produzca será el resultado.

## Ejemplos

| Entrada | Salida |
|---|---|
| `signo(42)` | `1` |
| `signo(-7)` | `-1` |
| `signo(0)` | `0` |

<details><summary>Pista 1</summary>

Un `if` es una expresión: `if n > 0 { 1 } else if ... `.
</details>

<details><summary>Pista 2</summary>

No olvides el `else` final: cada camino debe dar un `i32` (si no, error E0317).
</details>
