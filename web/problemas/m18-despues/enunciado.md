---
titulo: "Listo después de N sondeos"
mundo: 18-async
tema: async
dificultad: facil
---

Implementa el trait `Future` para `Despues`, un future que devuelve `Poll::Pending` las primeras `n` veces que lo sondean y, en el sondeo siguiente, `Poll::Ready(valor)`.

Cada vez que devuelvas `Pending` tienes que **avisar al waker** (`cx.waker().wake_by_ref()`), porque si no, un ejecutor de verdad no te volvería a sondear.

```text
despues(0, 7) → Ready(7) en el primer sondeo
despues(3, 7) → Pending, Pending, Pending, Ready(7)
```

<details><summary>Pista 1</summary>

`Despues` es `Unpin` (solo tiene un `u32` y un `u64`), así que dentro de `poll` puedes modificar sus campos con `self.n -= 1` si declaras `mut self: Pin<&mut Self>`.

</details>

<details><summary>Pista 2</summary>

Orden: si `n == 0`, devuelve `Ready(self.valor)`. Si no: resta 1, llama a `cx.waker().wake_by_ref()` y devuelve `Pending`.

</details>
