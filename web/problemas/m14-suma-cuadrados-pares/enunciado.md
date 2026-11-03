---
titulo: "Suma de cuadrados de los pares"
mundo: 14-closures-e-iteradores
tema: iteradores
dificultad: facil
---

Escribe `suma_cuadrados_pares(v: &[i64]) -> i64`: eleva al cuadrado cada número **par** de la lista y suma los resultados. Hazlo con una sola cadena de iterador, sin bucles.

```text
suma_cuadrados_pares(&[1, 2, 3, 4]) → 20     (4 + 16)
suma_cuadrados_pares(&[])           → 0
suma_cuadrados_pares(&[-3, -2])     → 4
```

<details><summary>Pista 1</summary>

`v.iter()` entrega `&i64`. En `filter`, el closure recibe `&&i64`: el patrón `|&&x|` te deja `x` como `i64`.

</details>

<details><summary>Pista 2</summary>

`filter`, después `map`, y al final `sum()`: el tipo de retorno de la función le dice a `sum` que sume en `i64`.

</details>
