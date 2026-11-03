---
titulo: "Figuras con un trait"
mundo: 13-traits-y-genericos
tema: traits
dificultad: facil
---

Define el trait `Figura` con dos métodos:

- `fn area(&self) -> f64;`
- `fn nombre(&self) -> String` con implementación **por defecto** que devuelva `"figura"`.

Impleméntalo para `Rectangulo { ancho, alto }` (área = ancho × alto, nombre `"rectángulo"`) y para `Circulo { radio }` (área = π × r², sin cambiar el nombre por defecto).

Después escribe `area_total(figuras: &[&dyn Figura]) -> f64`.
