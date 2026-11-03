---
titulo: "La primera pareja"
mundo: 04-expresiones-y-control
tema: etiquetas
dificultad: dificil
---

Escribe el cuerpo de `primera_pareja`: busca dos cifras `a` y `b` con `1 ≤ a ≤ b ≤ 9` cuyo producto sea `objetivo`. Recorre `a` de menor a mayor y, para cada `a`, `b` desde `a` hasta 9; devuelve la **primera** pareja `(a, b)` que encuentres. Si no hay ninguna, devuelve `(0, 0)`.

## Ejemplos

| Entrada | Salida |
|---|---|
| `primera_pareja(12)` | `(2, 6)` |
| `primera_pareja(81)` | `(9, 9)` |
| `primera_pareja(11)` | `(0, 0)` |

<details><summary>Pista 1</summary>

Dos `for` anidados. Para salir de los dos a la vez con un resultado, usa un **bloque etiquetado**: `'buscar: { ... break 'buscar (a, b); ... (0, 0) }`.
</details>

<details><summary>Pista 2</summary>

El bucle interior empieza en `a`: `for b in a..=9`. Así nunca devuelves `(6, 2)` en lugar de `(2, 6)`.
</details>
