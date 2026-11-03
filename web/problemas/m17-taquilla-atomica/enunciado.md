---
titulo: "Una taquilla atómica"
mundo: 17-concurrencia
tema: atomicos
dificultad: media
---

Una taquilla vende entradas desde varios hilos a la vez. `Taquilla` guarda las entradas que quedan en un `AtomicU32`.

Escribe `vender(&self, n: u32) -> bool`: si quedan al menos `n`, las descuenta y devuelve `true`; si no, no cambia nada y devuelve `false`. Tiene que ser correcto aunque lo llamen varios hilos a la vez: **comprobar y restar tiene que ser una sola operación atómica**.

(Las pruebas se ejecutan en un solo hilo, porque en el navegador no hay hilos, pero tu solución tiene que valer para varios.)

<details>
<summary>Pista</summary>

Un `load` seguido de un `fetch_sub` no vale: otro hilo podría vender entre los dos. Usa `fetch_update(Ordering::AcqRel, Ordering::Acquire, |q| q.checked_sub(n))`, que reintenta solo con `compare_exchange` hasta que lo consigue o la closure devuelve `None`.
</details>
