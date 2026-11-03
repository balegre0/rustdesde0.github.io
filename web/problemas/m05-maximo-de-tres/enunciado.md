---
titulo: "El mayor de tres"
mundo: 05-funciones-y-la-pila
tema: funciones
dificultad: facil
---

Escribe `maximo3`, que devuelve el mayor de tres números enteros.

## Ejemplos

| Entrada | Salida |
|---|---|
| `maximo3(4, 11, 6)` | `11` |
| `maximo3(-2, -9, -5)` | `-2` |
| `maximo3(7, 7, 7)` | `7` |

<details><summary>Pista</summary>

Puedes escribir una función auxiliar `maximo(a, b)` y usarla dos veces: `maximo(maximo(a, b), c)`.
</details>
