---
titulo: "La más larga de la lista"
mundo: 07-prestamos
tema: prestamos
dificultad: media
---

Escribe `mas_larga_de`: recibe un slice prestado de `String` y devuelve una **vista** (`&str`) del texto con más caracteres (en empate, el primero), o `None` si el slice está vacío. No clones nada.

## Ejemplos

| Entrada | Resultado |
|---|---|
| `["uno", "tres", "dos"]` | `Some("tres")` |
| `["añil", "azul"]` | `Some("añil")` (los dos tienen 4 caracteres) |
| `[]` | `None` |

<details>
<summary>Pista 1</summary>

Por la regla de elisión, `fn mas_larga_de(v: &[String]) -> Option<&str>` significa que la vista devuelta vive tanto como el préstamo `v`. Un `&String` se convierte en `&str` con `.as_str()`.
</details>

<details>
<summary>Pista 2</summary>

Guarda la mejor como `let mut mejor: Option<&str> = None;` y compárala con cada `s.chars().count()`.
</details>
