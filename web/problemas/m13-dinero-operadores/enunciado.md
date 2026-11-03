---
titulo: "Sumar dinero"
mundo: 13-traits-y-genericos
tema: operadores
dificultad: media
---

`Euros` guarda una cantidad en céntimos (`i64`) para no perder precisión. Implementa:

- `Add` (`a + b`) y `Sub` (`a - b`), que devuelven `Euros`.
- `Mul<i64>` (`a * 3`), que multiplica la cantidad.
- `Display`, con el formato `"12,05 €"` (coma decimal, siempre dos cifras de céntimos). Para cantidades negativas: `"-3,50 €"`.

<details>
<summary>Pista</summary>

Para `Display`, separa el signo y trabaja con el valor absoluto (`self.0.abs()`): euros = `n / 100`, céntimos = `n % 100`, con `{:02}`.
</details>
