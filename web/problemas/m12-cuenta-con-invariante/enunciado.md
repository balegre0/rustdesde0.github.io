---
titulo: "Una cuenta que no puede quedar en negativo"
mundo: 12-modulos-crates-y-cargo
tema: visibilidad
dificultad: media
---

En el módulo `banco`, `Cuenta` tiene un campo **privado** `saldo: u64`, así que nadie puede dejarla en un estado imposible desde fuera. Completa sus métodos:

- `Cuenta::nueva() -> Cuenta`: saldo 0.
- `ingresar(&mut self, n: u64)`: suma `n`.
- `retirar(&mut self, n: u64) -> Result<(), String>`: si hay saldo suficiente, lo resta; si no, devuelve `Err("saldo insuficiente: hay X, pides Y")` sin cambiar nada.
- `saldo(&self) -> u64`.

<details>
<summary>Pista</summary>

`self.saldo.checked_sub(n)` devuelve `None` si la resta se quedaría por debajo de cero.
</details>
