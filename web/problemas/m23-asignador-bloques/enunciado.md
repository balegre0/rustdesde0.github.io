---
titulo: "Un asignador de bloques fijos"
mundo: 23-embebido-y-tu-so
tema: memoria
dificultad: dificil
---

Un asignador de **bloques fijos** divide su memoria en 8 bloques del mismo tamaño y lleva en un `u8` cuáles están ocupados (bit `i` a 1 = bloque `i` ocupado).

Completa `Bloques`:

- `reservar(&mut self) -> Option<usize>`: ocupa el **primer** bloque libre y devuelve su índice, o `None` si están todos ocupados.
- `liberar(&mut self, i: usize) -> Result<(), &'static str>`: libera el bloque `i`. Si `i >= 8`: `Err("fuera de rango")`. Si ya estaba libre: `Err("doble liberación")` (es justo el error que el ownership impide en Rust seguro).
- `libres(&self) -> u32`: cuántos bloques quedan libres.

<details>
<summary>Pista</summary>

`(!self.ocupados).trailing_zeros()` da la posición del primer bit a 0. `count_zeros()` cuenta los libres.
</details>
