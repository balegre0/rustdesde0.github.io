---
titulo: "La mediana y sus casos límite"
mundo: 16-pruebas-y-herramientas
tema: pruebas
dificultad: media
---

La **mediana** de una lista de números es el valor del medio cuando la ordenas. Si la lista tiene un número par de elementos, es la media de los dos del medio.

Escribe `mediana(v: &[f64]) -> Option<f64>`:

- lista vacía → `None`;
- no debe modificar la lista original (la recibes prestada: ordena una copia);
- puedes suponer que no hay `NaN`.

Las pruebas cubren listas desordenadas, con repetidos, con un solo elemento y con negativos, y comprueban una **propiedad**: la mediana no cambia si barajas la lista.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `mediana(&[3.0, 1.0, 2.0])` | `Some(2.0)` |
| `mediana(&[4.0, 1.0, 3.0, 2.0])` | `Some(2.5)` |
| `mediana(&[])` | `None` |

<details>
<summary>Pista 1</summary>

`let mut copia = v.to_vec();` crea un `Vec<f64>` propio. Los `f64` no implementan `Ord` (por culpa de `NaN`), así que `copia.sort()` no compila: usa `copia.sort_by(|a, b| a.total_cmp(b));`.
</details>

<details>
<summary>Pista 2</summary>

Con `n = copia.len()`: si `n` es impar, la mediana es `copia[n / 2]`; si es par, la media de `copia[n / 2 - 1]` y `copia[n / 2]`.
</details>
