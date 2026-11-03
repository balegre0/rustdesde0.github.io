---
titulo: "Una tabla bien alineada"
mundo: 02-tu-primer-programa
tema: formato
dificultad: facil
---

Escribe `tabla(n: u32) -> String` que devuelva la tabla de multiplicar de `n` del 1 al 10, una línea por fila, con este formato exacto (los números alineados a la derecha en anchos de 2, 2 y 3):

```text
 7 x  1 =   7
 7 x  2 =  14
…
 7 x 10 =  70
```

Cada línea termina en un salto de línea `\n`, también la última.

<details>
<summary>Pista</summary>

`format!("{:>2} x {:>2} = {:>3}\n", n, i, n * i)` alinea a la derecha. Ve juntando las líneas en un `String` con `push_str`.
</details>
