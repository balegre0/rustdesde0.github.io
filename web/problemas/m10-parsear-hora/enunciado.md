---
titulo: "Parsear una hora"
mundo: 10-errores
tema: errores
dificultad: media
---

Escribe `parsear_hora(texto: &str) -> Result<Hora, ErrorHora>` para textos con el formato `HH:MM`.

- Si no hay exactamente un `:` o alguna parte no es un número (`u8`) → `Err(ErrorHora::Formato)`.
- Si la hora es mayor que 23 → `Err(ErrorHora::HoraInvalida(h))`.
- Si los minutos son mayores que 59 → `Err(ErrorHora::MinutoInvalido(m))`.
- Si todo va bien → `Ok(Hora { h, m })`.

Además, implementa `Display` para `ErrorHora` con estos textos exactos:

| Variante | Texto |
|---|---|
| `Formato` | `usa el formato HH:MM` |
| `HoraInvalida(h)` | `la hora h no existe` |
| `MinutoInvalido(m)` | `el minuto m no existe` |

<details>
<summary>Pista 1</summary>

`texto.split_once(':')` devuelve `Option<(&str, &str)>`. Con `let Some((a, b)) = … else { return Err(ErrorHora::Formato) };` tienes las dos partes. Si `b` contiene otro `:`, el `parse` de `b` fallará solo.
</details>

<details>
<summary>Pista 2</summary>

`a.parse::<u8>().map_err(|_| ErrorHora::Formato)?` convierte cualquier fallo de número en `Formato`.
</details>
