---
titulo: "Una invariante que no se rompe"
mundo: 20-unsafe-y-ub
tema: unsafe
dificultad: dificil
---

`TextoAscii` guarda bytes que **siempre** son ASCII. Esa es su *invariante de seguridad*: gracias a ella,
`como_str` puede usar `std::str::from_utf8_unchecked` (que exige UTF-8 válido) sin comprobar nada.

Completa los tres métodos **sin romper nunca la invariante**:

- `nuevo(s)`: `Some` si `s` es todo ASCII; `None` si no.
- `empujar(c)`: añade `c` si es ASCII y devuelve `true`; si no, no cambia nada y devuelve `false`.
- `como_str()`: devuelve el texto como `&str` usando `from_utf8_unchecked` con su `// SAFETY:`.

El campo `bytes` es privado al módulo: ese es el muro que protege la invariante. Pregúntate qué
método público, por inocente que parezca, la rompería (por ejemplo, uno que devolviera `&mut Vec<u8>`).

**Ejemplo**

```text
nuevo("Hola")  → Some(..), como_str() = "Hola"
nuevo("año")   → None
empujar('!')   → true;  empujar('ñ') → false
```

<details><summary>Pista</summary>

`str::is_ascii`, `char::is_ascii`, y `c as u8` (válido para un `char` ASCII).
</details>
