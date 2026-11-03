---
titulo: "Un búfer circular sin heap"
mundo: 23-embebido-y-tu-so
tema: no_std
dificultad: media
---

En embebido, los datos que llegan por el puerto serie se guardan en un **búfer circular** de tamaño fijo, sin memoria dinámica. Completa `Anillo` (capacidad 4, elementos `u8`):

- `meter(&mut self, b: u8) -> Result<(), u8>`: lo añade al final; si está lleno, devuelve `Err(b)` (el byte se pierde).
- `sacar(&mut self) -> Option<u8>`: saca el más antiguo.
- `len(&self) -> usize`.

Usa solo el array `datos`, el índice `inicio` y la longitud `n`: sin `Vec` ni `VecDeque`.

<details>
<summary>Pista</summary>

La posición donde escribir es `(inicio + n) % 4`. Al sacar, el dato está en `inicio`, que avanza con `(inicio + 1) % 4`.
</details>
