---
titulo: "La máquina de pila"
mundo: 21-el-compilador-por-dentro
tema: compilador
dificultad: facil
---

El último paso del mini-compilador del jefe es **ejecutar** el código generado. Escribe `ejecutar(prog: &[Instr]) -> Result<i64, String>` para esta máquina:

- `Empuja(n)`: mete `n` en la pila.
- `Suma`, `Resta`, `Mul`, `Div`: sacan `b` (cima) y luego `a`, y meten `a op b`.
- Al terminar, la pila debe tener **exactamente un** valor: ese es el resultado.

Errores (devuelve exactamente estos textos):

- Sacar de una pila vacía: `"pila vacía"`.
- Dividir entre cero: `"división por cero"`.
- Terminar con 0 o con más de un valor: `"la pila termina con N valores"`.

Usa operaciones con comprobación (`checked_add`, `checked_mul`…) y, si desbordan, devuelve `"desbordamiento"`.

Ejemplo: `[Empuja(7), Empuja(2), Resta]` → `Ok(5)`.

<details><summary>Pista</summary>

Una función auxiliar `fn sacar(pila: &mut Vec<i64>) -> Result<i64, String>` con `pila.pop().ok_or(...)` te ahorra mucho código, y con `?` propagas el error.

</details>
