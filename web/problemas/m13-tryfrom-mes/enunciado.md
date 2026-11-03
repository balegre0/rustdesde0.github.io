---
titulo: "Del número al mes"
mundo: 13-traits-y-genericos
tema: conversiones
dificultad: media
---

Implementa `TryFrom<u8> for Mes` (1 = `Enero` … 12 = `Diciembre`). Si el número no es un mes válido, el error es `MesInvalido(n)`.

Así, `Mes::try_from(3)` da `Ok(Mes::Marzo)` y `13u8.try_into()` da `Err(MesInvalido(13))`.

<details>
<summary>Pista</summary>

Un array con los doce meses en orden y `MESES.get((n as usize).wrapping_sub(1))` te dan el mes o `None`.
</details>
