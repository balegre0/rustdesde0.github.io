---
titulo: "Darle la vuelta a una lista en compilación"
mundo: 19-macros
tema: macros
dificultad: dificil
---

Escribe la macro `al_reves!` que reciba expresiones separadas por comas (con coma final opcional) y se expanda a un **array** con esas expresiones en orden inverso.

```rust
al_reves![1, 2, 3]       // [3, 2, 1]
al_reves!["a"]           // ["a"]
```

No vale construir el array en orden y llamar a `.reverse()`: el array generado ya debe estar al revés (las pruebas lo comprueban en una constante, donde `.reverse()` no está disponible).

<details><summary>Pista 1</summary>

Usa una regla interna con dos listas: `(@ [pendientes...] [invertidos...])`.
</details>

<details><summary>Pista 2</summary>

En cada paso, quita el primer pendiente y ponlo **al principio** de los invertidos. Cuando no queden pendientes, devuelve `[invertidos...]`. Pon las reglas `@` antes que la regla de entrada.
</details>
