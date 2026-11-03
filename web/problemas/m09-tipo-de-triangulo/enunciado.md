---
titulo: "¿Qué triángulo es?"
mundo: 09-structs-enums-y-patrones
tema: patrones
dificultad: facil
---

Escribe `clasificar(a, b, c)`, que recibe las longitudes de los tres lados (`u32`) y devuelve una variante del enum `Triangulo`:

- `Invalido` si algún lado es 0 o si un lado es mayor o igual que la suma de los otros dos.
- `Equilatero` si los tres lados son iguales.
- `Isosceles` si exactamente dos son iguales.
- `Escaleno` si los tres son distintos.

El enum ya tiene `#[derive(Debug, PartialEq)]` para que las pruebas puedan compararlo e imprimirlo.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `clasificar(3, 3, 3)` | `Equilatero` |
| `clasificar(3, 3, 5)` | `Isosceles` |
| `clasificar(3, 4, 5)` | `Escaleno` |
| `clasificar(1, 2, 3)` | `Invalido` (1 + 2 no es mayor que 3) |

<details>
<summary>Pista 1</summary>

Comprueba primero el caso inválido y sal con `return`. Después, un `match` sobre la tupla `(a == b, b == c, a == c)` resuelve el resto.
</details>

<details>
<summary>Pista 2</summary>

Con `(true, true, _)` ya sabes que los tres son iguales. Cualquier otro `true` significa dos iguales.
</details>
