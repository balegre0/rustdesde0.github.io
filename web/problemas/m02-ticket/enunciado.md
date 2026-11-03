---
titulo: "El ticket de la compra"
mundo: 02-tu-primer-programa
tema: formato
dificultad: media
---

Escribe `linea_ticket(producto: &str, unidades: u32, precio: f64) -> String` con este formato exacto (sin salto de línea al final):

- el nombre del producto alineado a la **izquierda** en 12 caracteres;
- `x` y las unidades alineadas a la derecha en 3;
- el total (`unidades × precio`) con **2 decimales**, alineado a la derecha en 8, seguido de ` €`.

Ejemplo: `linea_ticket("pan", 2, 1.25)` → `"pan          x  2     2.50 €"`.

Y escribe `total(lineas: &[(&str, u32, f64)]) -> String`: `TOTAL` alineado a la izquierda en 12, 6 espacios (el sitio de ` x  2 `) y la suma con el mismo formato que los totales, para que mida **exactamente lo mismo** que una línea: `"TOTAL                13.40 €"`.

<details>
<summary>Pista</summary>

`{:<12}` alinea a la izquierda, `{:>3}` a la derecha y `{:>8.2}` a la derecha con 2 decimales. Para que «TOTAL» quede alineado, rellena el hueco de ` x  2 ` (6 caracteres) con `{:>6}` y un texto vacío.
</details>
