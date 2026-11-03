---
titulo: "Dentro del rango"
mundo: 16-pruebas-y-herramientas
tema: pruebas
dificultad: facil
---

Escribe `en_rango(x, min, max)`, que dice si `x` está entre `min` y `max`, **ambos incluidos**. Si `min > max`, el rango está vacío y la respuesta es siempre `false`.

Las pruebas insisten en los **bordes**: justo en `min`, justo en `max`, uno por debajo y uno por encima. Es donde viven los errores de «uno de más o uno de menos».

## Ejemplos

| Llamada | Resultado |
|---|---|
| `en_rango(5, 1, 10)` | `true` |
| `en_rango(1, 1, 10)` | `true` |
| `en_rango(11, 1, 10)` | `false` |
| `en_rango(3, 5, 2)` | `false` |

<details>
<summary>Pista</summary>

Hay un rango inclusivo en la biblioteca estándar: `(min..=max).contains(&x)`. Y si lo escribes a mano, revisa si tus comparaciones son `<` o `<=`.
</details>
