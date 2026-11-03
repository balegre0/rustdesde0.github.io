---
titulo: "Llaves dentro del formato"
mundo: 02-tu-primer-programa
tema: formato
dificultad: facil
---

Escribe `json_simple(nombre: &str, edad: u32) -> String` que devuelva exactamente:

```text
{"nombre": "Ana", "edad": 30}
```

(con los valores que reciba). Tienes que usar `format!`: recuerda que en una cadena de formato, una llave literal se escribe **doble** (`{{` y `}}`), y que `{:?}` de un `&str` lo pone entre comillas.

<details>
<summary>Pista</summary>

`format!("{{\"nombre\": {:?}, \"edad\": {}}}", nombre, edad)`. Ojo con el final: `}` del argumento seguido de `}}` de la llave literal.
</details>
