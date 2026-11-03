---
titulo: "Analizar con precedencia"
mundo: 21-el-compilador-por-dentro
tema: compilador
dificultad: dificil
---

Escribe un analizador y evaluador de expresiones con enteros, `+`, `-`, `*`, `/`, paréntesis y espacios: `evaluar(texto: &str) -> Result<i64, String>`.

- `*` y `/` tienen más precedencia que `+` y `-`, y todos se asocian por la izquierda: `10 - 3 - 2` es `5`, `2 + 3 * 4` es `14`.
- División entera; dividir entre 0 → `Err("división entre cero")`.
- Cualquier entrada mal formada (un carácter extraño, paréntesis sin cerrar, falta un número, sobra algo al final) → `Err` con el mensaje que quieras.

Es exactamente lo que hace la fase de *parsing* de `rustc` con las expresiones, en pequeño.

<details>
<summary>Pista</summary>

Descenso recursivo con una función por nivel de precedencia: `expresion` (sumas y restas) llama a `termino` (productos y divisiones), que llama a `factor` (un número o algo entre paréntesis). Cada una lee de un `Peekable<Chars>` compartido. Al final, comprueba que no queda nada sin leer.
</details>
