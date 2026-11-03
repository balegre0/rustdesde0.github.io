---
titulo: "Área y perímetro"
mundo: 05-funciones-y-la-pila
tema: funciones
dificultad: facil
---

Escribe `rectangulo`, que recibe el ancho y el alto de un rectángulo y devuelve **dos** valores en una tupla: `(área, perímetro)`.

## Ejemplos

| Entrada | Salida |
|---|---|
| `rectangulo(4, 3)` | `(12, 14)` |
| `rectangulo(5, 5)` | `(25, 20)` |
| `rectangulo(0, 7)` | `(0, 14)` |

<details><summary>Pista</summary>

La última expresión del cuerpo puede ser una tupla: `(ancho * alto, 2 * (ancho + alto))`. ¡Sin punto y coma!
</details>
