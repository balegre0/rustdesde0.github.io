---
titulo: "Un intérprete de MIR de juguete"
mundo: 21-el-compilador-por-dentro
tema: mir
dificultad: dificil
---

El evaluador de constantes de rustc es un **intérprete de MIR**. Escribe uno pequeño.

Una función es una lista de bloques básicos. Cada bloque tiene sentencias sobre 8 locales `i64` (`Asigna(destino, operando)`, `Suma(destino, a, b)`, `Mul(destino, a, b)`) y un **terminador**:

- `Goto(b)`: salta al bloque `b`.
- `SwitchInt(local, valor, si_igual, si_no)`: si `locales[local] == valor` salta a `si_igual`; si no, a `si_no`.
- `Assert(local, mensaje)`: si `locales[local] == 0`, devuelve `Err(mensaje)`; si no, sigue en el bloque siguiente (índice + 1).
- `Return`: devuelve `Ok(locales[0])`.

`ejecutar(bloques, args)` empieza en el bloque 0 con los 8 locales a cero, salvo `_1`, `_2`… que reciben los argumentos. Si se ejecutan 10 000 terminadores sin llegar a `Return`, devuelve `Err("demasiados pasos")`.

Las pruebas incluyen un factorial escrito como bloques: `_0 = 1`, y mientras `_1 != 0`, `_0 = _0 * _1` y `_1 = _1 + (-1)`.

<details><summary>Pista</summary>

Una variable `actual: usize` con el bloque en curso, un bucle, y un `match` sobre el terminador que calcula el siguiente `actual`: esa es la forma de cualquier intérprete de MIR.

</details>
