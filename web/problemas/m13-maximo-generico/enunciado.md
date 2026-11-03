---
titulo: "El máximo de cualquier cosa"
mundo: 13-traits-y-genericos
tema: genericos
dificultad: media
---

Escribe una función genérica `maximo<T>(xs: &[T]) -> Option<&T>` que devuelva una referencia al mayor elemento, o `None` si el slice está vacío. Si hay varios iguales al máximo, devuelve el **primero**.

Elige tú la cota mínima que necesita `T` (no pidas `Copy` ni `Clone`: así funcionará también con `String`).
