---
titulo: "Romanos de ida y vuelta"
mundo: 16-pruebas-y-herramientas
tema: pruebas
dificultad: media
---

Escribe `a_romano(n: u32) -> String` (para 1 ≤ n ≤ 3999) y `de_romano(s: &str) -> Option<u32>`.

`de_romano` acepta solo romanos en forma **canónica** (la que produce `a_romano`): `"IIII"` o `"IC"` devuelven `None`.

Las pruebas incluyen una **prueba de propiedad**: para **todos** los números del 1 al 3999, `de_romano(&a_romano(n)) == Some(n)`. En vez de inventar casos, se comprueba una propiedad que tiene que cumplirse siempre.

<details>
<summary>Pista</summary>

Para `a_romano`, una tabla de pares de mayor a menor que incluya los casos de resta: `(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), …, (4, "IV"), (1, "I")`. Para `de_romano`, calcula el valor de forma sencilla (sumando, y restando cuando un símbolo es menor que el siguiente) y después comprueba que `a_romano(valor) == s`: así rechazas las formas no canónicas.
</details>
