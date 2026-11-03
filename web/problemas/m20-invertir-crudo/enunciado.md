---
titulo: "Invertir con dos punteros"
mundo: 20-unsafe-y-ub
tema: unsafe
dificultad: media
---

Escribe `invertir<T>(v: &mut [T])`, que da la vuelta al slice **en el sitio**, usando dos punteros
crudos: uno que avanza desde el principio y otro que retrocede desde el final, e intercambiando con
`std::ptr::swap` mientras no se crucen.

Debe funcionar con cualquier `T` (también con `String`, que tiene destructor): `ptr::swap` mueve los
bits sin destruir ni duplicar nada.

**Ejemplo**

```text
[1, 2, 3, 4, 5]   →   [5, 4, 3, 2, 1]
["a", "b"]        →   ["b", "a"]
[]                →   []
```

No puedes usar `v.reverse()` ni `v.swap(..)`.

<details><summary>Pista</summary>

Con `v.len() < 2` no hay nada que hacer (y así evitas calcular `p.add(len - 1)` con `len = 0`).
Luego: `let mut i = v.as_mut_ptr();`, `let mut j = i.add(len - 1);` y `while i < j { ... }`.
</details>
