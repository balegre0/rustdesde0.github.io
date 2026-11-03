---
titulo: "La cola de turnos"
mundo: 11-colecciones-y-texto
tema: colecciones
dificultad: media
---

Simula la cola de una ventanilla. Recibes una lista de órdenes:

- `"llega NOMBRE"`: NOMBRE se pone al final de la cola.
- `"urgente NOMBRE"`: NOMBRE se pone al **principio**.
- `"atiende"`: se atiende al primero de la cola (si la cola está vacía, no pasa nada).

Escribe `atendidos(ordenes: &[&str]) -> (Vec<String>, Vec<String>)` que devuelva quién fue atendido, en orden, y quién queda en la cola al final, en orden.

<details>
<summary>Pista</summary>

`VecDeque` tiene `push_back`, `push_front` y `pop_front`, todos O(1). Para separar la orden del nombre: `orden.split_once(' ')`.
</details>
