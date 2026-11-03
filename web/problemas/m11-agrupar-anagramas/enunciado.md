---
titulo: "Agrupar anagramas"
mundo: 11-colecciones-y-texto
tema: colecciones
dificultad: dificil
---

Dos palabras son **anagramas** si tienen las mismas letras en otro orden (`"amor"` y `"roma"`).

Escribe `agrupar(palabras: &[&str]) -> Vec<Vec<String>>` que agrupe las palabras que son anagramas entre sí.

- Dentro de cada grupo, las palabras van en orden alfabético.
- Los grupos van ordenados por su primera palabra.
- Compara letras exactas (sin cambiar mayúsculas), por caracteres.

Ejemplo: `["roma", "sol", "amor", "los", "mar"]` → `[["amor", "roma"], ["los", "sol"], ["mar"]]`.

<details>
<summary>Pista</summary>

La «firma» de una palabra son sus caracteres ordenados: `let mut c: Vec<char> = p.chars().collect(); c.sort();`. Usa un `HashMap<Vec<char>, Vec<String>>` y, al final, ordena cada grupo y la lista de grupos.
</details>
