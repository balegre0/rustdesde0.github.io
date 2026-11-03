---
titulo: "Esperar a todos"
mundo: 18-async
tema: async
dificultad: media
---

Escribe `async fn join_todos<F: Future + Unpin>(futs: Vec<F>) -> Vec<F::Output>`: espera a **todos** los futures de la lista **a la vez** (sondeándolos por turnos, no uno detrás de otro) y devuelve sus resultados **en el mismo orden** en que llegaron, aunque terminen en otro orden.

Reglas:
- No vuelvas a sondear un future que ya devolvió `Ready`.
- Con una lista vacía, devuelve `vec![]` enseguida.

```text
join_todos(vec![tarda(3, 'a'), tarda(0, 'b'), tarda(1, 'c')]) → ['a', 'b', 'c']
```

Las pruebas cuentan los sondeos: con futures que tardan 3, 0 y 1 sondeos, el conjunto debe terminar en **4** sondeos (lo que tarda el más lento + 1), no en 7.

<details><summary>Pista 1</summary>

Guarda los resultados en un `Vec<Option<F::Output>>` con un `None` por future, y crea el future con `std::future::poll_fn(move |cx| { ... })`.

</details>

<details><summary>Pista 2</summary>

Como `F: Unpin`, puedes sondear cada uno con `Pin::new(&mut futs[i]).poll(cx)`. Cuando todos los `Option` sean `Some`, devuelve `Poll::Ready` con los valores (sácalos con `take()`).

</details>
