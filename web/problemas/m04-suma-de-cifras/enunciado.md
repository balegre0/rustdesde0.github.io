---
titulo: "Suma de cifras"
mundo: 04-expresiones-y-control
tema: while
dificultad: media
---

Escribe el cuerpo de `suma_cifras`: devuelve la suma de las cifras decimales de `n`.

## Ejemplos

| Entrada | Salida |
|---|---|
| `suma_cifras(0)` | `0` |
| `suma_cifras(4096)` | `19` |
| `suma_cifras(999)` | `27` |

<details><summary>Pista 1</summary>

`n % 10` es la última cifra; `n / 10` quita la última cifra.
</details>

<details><summary>Pista 2</summary>

Usa `let mut resto = n;` y un `while resto > 0 { ... }`.
</details>
