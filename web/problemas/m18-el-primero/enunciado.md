---
titulo: "El primero que llegue"
mundo: 18-async
tema: cancelacion
dificultad: dificil
---

Escribe `async fn el_primero<F: Future + Unpin>(futs: Vec<F>) -> Option<(usize, F::Output)>`, un `select` para listas:

- Sondea los futures por turnos, en orden de índice.
- En cuanto uno devuelva `Ready(v)`, termina con `Some((indice, v))`. Si en una misma vuelta terminarían varios, gana el de **menor índice** (el primero que sondeas).
- Los demás quedan **cancelados**: se sueltan sin volver a sondearlos.
- Con una lista vacía, devuelve `None`.

```text
el_primero(vec![tarda(4, "lento"), tarda(1, "rápido"), tarda(1, "empate")]) → Some((1, "rápido"))
el_primero(Vec::new())                                                    → None
```

Las pruebas comprueban también que los perdedores se sueltan (sus `Drop` se ejecutan) y que nadie se sondea después de que haya un ganador.

<details><summary>Pista 1</summary>

Si la lista está vacía, devuelve `None` antes de empezar. Si no, usa `std::future::poll_fn` y recorre la lista con `iter_mut().enumerate()`.

</details>

<details><summary>Pista 2</summary>

En cuanto un `Pin::new(f).poll(cx)` dé `Ready(v)`, devuelve `Poll::Ready(Some((i, v)))` **sin seguir el bucle**. Cuando `el_primero` termina, el `Vec` que guardaba la closure se suelta solo: esa es la cancelación.

</details>
