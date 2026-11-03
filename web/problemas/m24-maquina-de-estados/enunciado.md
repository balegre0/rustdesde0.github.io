---
titulo: "Estados que no se pueden mezclar"
mundo: 24-teoria-y-diseno
tema: diseno
dificultad: media
---

Un pedido pasa por estos estados: **pendiente** → **pagado** (con el importe) → **enviado** (con el número de seguimiento). También se puede **cancelar**, pero solo si está pendiente.

En vez de varios campos `bool` y `Option` que permitirían combinaciones imposibles, modela el estado con un `enum` y escribe:

- `pagar(self, importe: u32) -> Result<Pedido, Pedido>`
- `enviar(self, seguimiento: &str) -> Result<Pedido, Pedido>`
- `cancelar(self) -> Result<Pedido, Pedido>`

Cada método devuelve `Ok(nuevo_estado)` si la transición es válida y `Err(estado_original)` si no. Además, `describir(&self) -> String` con estos textos exactos: `"pendiente"`, `"pagado (N €)"`, `"enviado (CÓDIGO)"`, `"cancelado"`.
