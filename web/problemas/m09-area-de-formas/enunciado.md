---
titulo: "Área de formas"
mundo: 09-structs-enums-y-patrones
tema: enums
dificultad: facil
---

Tienes un enum `Forma` con tres variantes. Escribe la función `area`, que recibe una forma **prestada** y devuelve su área como `f64`:

- `Circulo { radio }` → π · radio² (usa `std::f64::consts::PI`).
- `Rectangulo { ancho, alto }` → ancho · alto.
- `Triangulo { base, altura }` → base · altura / 2.

## Ejemplos

| Llamada | Resultado |
|---|---|
| `area(&Forma::Rectangulo { ancho: 3.0, alto: 4.0 })` | `12.0` |
| `area(&Forma::Triangulo { base: 10.0, altura: 3.0 })` | `15.0` |
| `area(&Forma::Circulo { radio: 1.0 })` | `3.141592653589793` |

<details>
<summary>Pista 1</summary>

Un `match` sobre `f` con un brazo por variante. Como `f` es `&Forma`, los campos que saques serán `&f64`: puedes escribir `*radio` o dejar que la aritmética los desreferencie (`radio * radio` funciona con `&f64`).
</details>

<details>
<summary>Pista 2</summary>

Si te olvidas una variante, el compilador te lo dice con el error E0004. Eso es bueno: léelo.
</details>
