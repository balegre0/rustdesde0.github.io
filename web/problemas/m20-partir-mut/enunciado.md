---
titulo: "Tu propio split_at_mut"
mundo: 20-unsafe-y-ub
tema: unsafe
dificultad: media
---

Escribe `partir_mut(s, i)`: devuelve dos préstamos exclusivos, uno de `s[..i]` y otro de `s[i..]`.
El borrow checker no te dejará hacerlo con `&mut s[..i]` y `&mut s[i..]` a la vez (E0499), así que
tendrás que usar `as_mut_ptr`, `add` y `std::slice::from_raw_parts_mut` dentro de un bloque `unsafe`.

**Reglas**

- Si `i > s.len()`, entra en pánico con `assert!` **antes** de tocar punteros: sin esa comprobación
  tu función sería *unsound* (código seguro podría provocar comportamiento indefinido).
- No puedes llamar a `split_at_mut` ni a `split_at_mut_unchecked`.
- Escribe un comentario `// SAFETY:` que explique por qué los dos trozos no se solapan.

**Ejemplo**

```text
s = [1, 2, 3, 4, 5], i = 2   →   ([1, 2], [3, 4, 5])
s = [7], i = 0               →   ([], [7])
```

<details><summary>Pista 1</summary>

El segundo trozo empieza en `p.add(i)` y mide `s.len() - i`.
</details>

<details><summary>Pista 2</summary>

Guarda `s.len()` en una variable **antes** de llamar a `as_mut_ptr()`.
</details>
