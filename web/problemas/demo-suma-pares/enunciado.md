---
titulo: "Suma de los pares"
mundo: 03-valores-y-tipos
tema: iteradores
dificultad: facil
---

Escribe `suma_pares(v: &[i32]) -> i32`, que devuelva la suma de los números **pares** de la lista.

```text
suma_pares(&[1, 2, 3, 4]) → 6
suma_pares(&[])           → 0
```

<details><summary>Pista</summary>

Un número es par si `n % 2 == 0`. Recorre la lista con `for n in v`.

</details>
