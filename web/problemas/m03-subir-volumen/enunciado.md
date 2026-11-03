---
titulo: "El volumen que no pasa de 255"
mundo: 03-valores-y-tipos
tema: overflow
dificultad: media
---

Un altavoz guarda su volumen en un `u8` (0 = silencio, 255 = máximo). Cada pulsación del botón «+» sube el volumen **10 unidades**.

Escribe `subir_volumen(volumen: u8, pulsaciones: u8) -> u8`, que devuelve el volumen tras pulsar el botón `pulsaciones` veces. Si se pasa del máximo, el volumen se queda en **255**. Nunca debe haber pánico ni dar la vuelta: un altavoz que pasa de «casi al máximo» a «casi en silencio» sería un mal altavoz.

| Entrada | Salida |
|---|---|
| `subir_volumen(0, 3)` | `30` |
| `subir_volumen(240, 1)` | `250` |
| `subir_volumen(250, 1)` | `255` |
| `subir_volumen(10, 200)` | `255` |

Cuidado con el último caso: `200 × 10` ya no cabe en un `u8` **antes** de sumar nada.

<details>
<summary>Pista 1</summary>

Hay dos operaciones que pueden desbordar: la multiplicación `pulsaciones × 10` y la suma. Las dos tienen que saturar.
</details>

<details>
<summary>Pista 2</summary>

`volumen.saturating_add(pulsaciones.saturating_mul(10))`.
</details>
