---
titulo: "El iterador de Collatz"
mundo: 14-closures-e-iteradores
tema: iteradores
dificultad: media
---

La sucesión de Collatz empieza en un número `n > 0` y sigue esta regla: si el número es par, el siguiente es la mitad; si es impar, el siguiente es el triple más uno. Se termina al llegar a 1 (el 1 se incluye).

Implementa el trait `Iterator` para el struct `Collatz` (la función `collatz(n)` que lo crea ya está hecha), de modo que:

```text
collatz(6).collect::<Vec<_>>() → [6, 3, 10, 5, 16, 8, 4, 2, 1]
collatz(1).collect::<Vec<_>>() → [1]
collatz(27).count()            → 112
```

Como implementas `Iterator`, `count`, `max`, `collect`… funcionan solos.

<details><summary>Pista 1</summary>

El struct guarda el **siguiente** número a entregar como `Option<u64>`: `Some(n)` mientras quede algo y `None` cuando ya se entregó el 1.

</details>

<details><summary>Pista 2</summary>

En `next`: saca el valor actual (si es `None`, devuelve `None`); calcula el siguiente (`None` si el actual es 1) y guárdalo; devuelve `Some(actual)`. El operador `?` sobre un `Option` te ahorra el `match`.

</details>
