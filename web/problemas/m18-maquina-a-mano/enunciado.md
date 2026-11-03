---
titulo: "Sé el compilador: una máquina de estados"
mundo: 18-async
tema: async
dificultad: dificil
---

Esta función async:

```rust
async fn dos_pasos(x: u32) -> u32 {
    let a = doble(x).await;
    let b = doble(a).await;
    a + b
}
```

se convierte, por dentro, en una máquina de estados. Escríbela tú: implementa `Future` para el enum `DosPasos`, que ya tiene sus estados:

| Estado | Qué guarda |
|---|---|
| `Inicio { x }` | el argumento |
| `Primero { espera }` | el future del primer `doble(x)` |
| `Segundo { a, espera }` | el resultado `a` y el future de `doble(a)` |
| `Fin` | nada |

`doble(n)` (ya escrito) devuelve `Pending` una vez y después `Ready(n * 2)`. Por tanto, `dos_pasos(3)` debe dar `6 + 12 = 18` en **3 sondeos**, exactamente como la versión `async`.

Si alguien sondea `Fin`, entra en pánico (`panic!("sondeado tras terminar")`).

<details><summary>Pista 1</summary>

Escribe `poll` como un `loop { match &mut *self { ... } }`. En `Inicio`, pasa a `Primero` y **sigue el bucle** (sin devolver nada todavía): el primer sondeo ya tiene que sondear al primer `doble`.

</details>

<details><summary>Pista 2</summary>

En `Primero` y `Segundo`: sondea `espera` con `Pin::new(espera).poll(cx)` (`Doble` es `Unpin`). Si es `Pending`, devuelve `Pending`. Si es `Ready(v)`, cambia de estado con `*self = ...` y sigue (o devuelve `Ready(a + v)` en el segundo).

</details>
