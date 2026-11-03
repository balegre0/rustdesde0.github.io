---
titulo: "Transferencias sin interbloqueo"
mundo: 17-concurrencia
tema: mutex
dificultad: dificil
---

Cada cuenta tiene un id y un saldo protegido por un `Mutex`. Escribe `transferir(a: &Cuenta, b: &Cuenta, cantidad: u64) -> Result<(), String>`:

- Para evitar interbloqueos cuando dos hilos transfieren en sentidos opuestos a la vez, **toma siempre primero el candado de la cuenta con menor `id`**.
- Si `a` y `b` son la misma cuenta (mismo `id`), devuelve `Err("misma cuenta")` (tomar dos veces el mismo `Mutex` bloquearía el hilo).
- Si `a` no tiene saldo suficiente, `Err("saldo insuficiente")` sin cambiar nada.
- Si todo va bien, mueve la cantidad y devuelve `Ok(())`.

<details>
<summary>Pista</summary>

Ordena las dos referencias por id: `let (primera, segunda) = if a.id < b.id { (a, b) } else { (b, a) };`. Toma los dos guards y, después, decide cuál es el de `a` y cuál el de `b`.
</details>
