---
titulo: "Potencia rápida"
mundo: 05-funciones-y-la-pila
tema: recursion
dificultad: dificil
---

Escribe `potencia(base, exp)` de forma **recursiva**, pero haciendo como mucho unas 2 × log₂(exp) multiplicaciones en lugar de `exp`:

- `b^0 = 1`
- si `e` es par: `b^e = (b^(e/2))²`
- si `e` es impar: `b^e = b × b^(e−1)`

Las pruebas solo usan resultados que caben en un `u64`.

## Ejemplos

| Entrada | Salida |
|---|---|
| `potencia(2, 10)` | `1024` |
| `potencia(3, 13)` | `1594323` |
| `potencia(2, 63)` | `9223372036854775808` |

<details><summary>Pista 1</summary>

En el caso par, calcula `let mitad = potencia(base, exp / 2);` **una sola vez** y devuelve `mitad * mitad`. Si llamas dos veces, pierdes toda la ventaja.
</details>

<details><summary>Pista 2</summary>

¿Cuántos marcos de pila hay como máximo? Con `exp = 63`, unos 11: cada dos llamadas, el exponente se divide al menos entre 2.
</details>
