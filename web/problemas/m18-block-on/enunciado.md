---
titulo: "Un block_on que no malgasta sondeos"
mundo: 18-async
tema: async
dificultad: media
---

Escribe `block_on<F: Future>(fut: F) -> F::Output`, un ejecutor que **solo sondea cuando le han despertado**:

1. Crea un waker propio con el trait `Wake` que, al llamarse, levanta una bandera (`AtomicBool`).
2. Fija el future (`std::pin::pin!`).
3. En bucle: si la bandera está levantada, bájala y sondea; si sale `Ready`, devuelve el valor. La primera vez, la bandera empieza levantada.
4. Si la bandera está bajada y el future no ha terminado, nadie lo va a despertar (aquí no hay E/S real): entra en pánico con el mensaje `"tarea dormida para siempre"`. (Las pruebas no comprueban este caso, porque en el navegador un pánico detiene el programa entero.)

Las pruebas comprueban el resultado y **cuántas veces** sondeas: un future que se despierta dos veces en un mismo sondeo no debe provocar dos sondeos extra.

```text
block_on(async { 5 })                       → 5   (1 sondeo)
block_on(cede 3 veces y devuelve "fin")      → "fin" (4 sondeos)
```

<details><summary>Pista</summary>

`bandera.0.swap(false, Ordering::SeqCst)` lee y baja la bandera en un solo paso: devuelve `true` si estaba levantada.

</details>
