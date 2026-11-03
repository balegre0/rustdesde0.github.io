---
titulo: "Palabras largas en mayúsculas"
mundo: 14-closures-e-iteradores
tema: iteradores
dificultad: media
---

Escribe `palabras_largas(texto: &str, minimo: usize) -> Vec<String>`: devuelve, en orden, las palabras del texto que tienen **al menos `minimo` caracteres**, pasadas a mayúsculas.

Cuidado: cuenta **caracteres**, no bytes. «ñandú» tiene 5 caracteres aunque ocupe 7 bytes.

```text
palabras_largas("el ñandú corre mucho", 5) → ["ÑANDÚ", "CORRE", "MUCHO"]
palabras_largas("a b c", 2)                → []
palabras_largas("", 1)                     → []
```

<details><summary>Pista 1</summary>

`texto.split_whitespace()` es un iterador de `&str` con cada palabra (y sabe ignorar espacios repetidos).

</details>

<details><summary>Pista 2</summary>

`p.chars().count()` cuenta caracteres; `p.to_uppercase()` devuelve un `String` nuevo. Termina con `collect()`: el tipo de retorno le dice que construya un `Vec<String>`.

</details>
