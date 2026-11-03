---
titulo: "Una suma que no puede fallar"
mundo: 24-teoria-y-diseno
tema: tipos
dificultad: facil
---

El tipo `NoVacio<T>` guarda el primer elemento aparte (`cabeza`) y el resto en un `Vec` (`cola`), así que **no puede estar vacío**. Escribe:

- `minimo(&self) -> &T` (con `T: PartialOrd`), sin `Option`: siempre hay al menos un elemento. Si hay empate, el primero.
- `media(lista: &NoVacio<f64>) -> f64`, sin `Option`: no hay división entre cero posible.
- `NoVacio::desde(v: Vec<T>) -> Option<NoVacio<T>>`: el único sitio donde se comprueba.
