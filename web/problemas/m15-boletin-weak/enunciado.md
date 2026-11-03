---
titulo: "Un boletín que no retiene a nadie"
mundo: 15-punteros-inteligentes
tema: weak
dificultad: dificil
---

Un `Boletin` envía mensajes a sus suscriptores. Cada suscriptor es un **buzón** compartido: `Rc<RefCell<Vec<String>>>`. El dueño del buzón es quien se suscribe; el boletín solo debe **recordarlo**, sin mantenerlo vivo. Por eso guarda referencias débiles:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Buzon = Rc<RefCell<Vec<String>>>;

struct Boletin {
    suscriptores: Vec<Weak<RefCell<Vec<String>>>>,
}
```

Escribe estos métodos:

- `suscribir(&mut self, buzon: &Buzon)`: guarda un `Weak` al buzón.
- `publicar(&mut self, mensaje: &str) -> usize`: añade el mensaje a **cada buzón que siga vivo** y devuelve a cuántos llegó. Los buzones que ya no existen se **eliminan** de la lista.
- `activos(&self) -> usize`: cuántos suscriptores guardados siguen vivos.

```text
ana y luis se suscriben         publicar("hola")  → 2
luis se va (drop)               publicar("adiós") → 1   (y su Weak se borra de la lista)
                                suscriptores.len() == 1
```

<details><summary>Pista 1</summary>

`weak.upgrade()` devuelve `Some(rc)` si el buzón vive. Con ese `rc` puedes hacer `rc.borrow_mut().push(mensaje.to_string())`.

</details>

<details><summary>Pista 2</summary>

`Vec::retain` recorre la lista y se queda solo con los elementos para los que tu closure devuelve `true`. Dentro puedes enviar el mensaje y devolver si el buzón estaba vivo. Para `activos`, `Weak::strong_count(&w) > 0` o `w.upgrade().is_some()`.

</details>
