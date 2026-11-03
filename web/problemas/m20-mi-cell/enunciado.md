---
titulo: "Tu propia Cell"
mundo: 20-unsafe-y-ub
tema: unsafe
dificultad: media
---

Mutar a través de un préstamo compartido (`&T`) solo es legal si pasa por `UnsafeCell`. Construye
`MiCell<T: Copy>` con tres métodos que reciben `&self`:

- `get()`: devuelve una copia del valor.
- `set(v)`: lo sustituye.
- `reemplazar(v)`: lo sustituye y devuelve el valor anterior.

Usa `UnsafeCell::get()` (que da un `*mut T`) dentro de bloques `unsafe` con su `// SAFETY:`.
Tu tipo será automáticamente `!Sync` (porque `UnsafeCell` lo es), y eso es justo lo que hace que sea
sound: ningún otro hilo puede tocarlo a la vez.

**Regla de oro**: no devuelvas nunca `&T` ni `&mut T` al interior; solo copias.

<details><summary>Pista</summary>

`reemplazar` = leer con `*p`, escribir con `*p = v`, devolver lo leído (o `std::mem::replace(&mut *p, v)`).
</details>
