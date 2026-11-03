---
titulo: "Interpretar comandos"
mundo: 09-structs-enums-y-patrones
tema: patrones
dificultad: media
---

Un robot entiende órdenes de texto. Escribe `parsear(linea: &str) -> Option<Comando>`:

| Texto | Resultado |
|---|---|
| `"avanza 3"` | `Some(Comando::Avanzar(3))` |
| `"gira izquierda"` | `Some(Comando::Girar(Lado::Izquierda))` |
| `"gira derecha"` | `Some(Comando::Girar(Lado::Derecha))` |
| `"para"` | `Some(Comando::Parar)` |
| cualquier otra cosa (`"avanza"`, `"avanza tres"`, `"gira arriba"`, `""`, `"para ya"`) | `None` |

Las palabras van separadas por un espacio o más.

<details>
<summary>Pista 1</summary>

`linea.split_whitespace()` da las palabras una a una. Si las recoges en un `Vec<&str>` (`.collect::<Vec<&str>>()`), puedes hacer `match` sobre un slice: `match palabras.as_slice() { ["para"] => …, ["avanza", n] => …, _ => None }`.
</details>

<details>
<summary>Pista 2</summary>

Para convertir `"3"` en número: `n.parse::<u32>()` devuelve un `Result`; `.ok()` lo convierte en `Option`. Combínalo con `let Some(pasos) = … else { return None };` o con `.map(…)`.
</details>
