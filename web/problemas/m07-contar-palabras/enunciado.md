---
titulo: "Contar palabras"
mundo: 07-prestamos
tema: prestamos
dificultad: media
---

Escribe `contar`: recibe un texto prestado y un préstamo exclusivo de un `HashMap<String, u32>`, y suma 1 a la cuenta de cada palabra del texto (separadas por espacios en blanco). El mapa puede traer ya cuentas de antes.

## Ejemplos

Con el mapa vacío, `contar("el gato y el perro", &mut m)` deja `m["el"] == 2`, `m["gato"] == 1`, `m["y"] == 1`, `m["perro"] == 1`. Si después llamas a `contar("el fin", &mut m)`, `m["el"]` pasa a 3.

<details>
<summary>Pista 1</summary>

`texto.split_whitespace()` recorre las palabras como `&str`.
</details>

<details>
<summary>Pista 2</summary>

El patrón «buscar y, si no está, insertar» con `get` + `insert` choca con el borrow checker (E0502). Usa la API `entry`: `*mapa.entry(palabra.to_string()).or_insert(0) += 1;`
</details>
