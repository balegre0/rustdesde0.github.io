---
titulo: "Fábrica de contadores"
mundo: 14-closures-e-iteradores
tema: closures
dificultad: media
---

Escribe `hacer_contador(inicio, paso)`, que **devuelva un closure**. Cada vez que se llame al closure, debe devolver el siguiente valor de la cuenta: primero `inicio`, luego `inicio + paso`, luego `inicio + 2·paso`…

Cada contador es independiente: dos contadores creados por separado no comparten la cuenta.

```text
let mut c = hacer_contador(10, 5);
c() → 10
c() → 15
c() → 20
```

<details><summary>Pista 1</summary>

El closure modifica su propio estado en cada llamada: ¿qué trait de llamada implementa? El tipo de retorno ya lo dice: `impl FnMut() -> i32`.

</details>

<details><summary>Pista 2</summary>

La cuenta tiene que vivir **dentro** del closure, no en la función, que termina al devolverlo: hace falta `move`.

</details>
