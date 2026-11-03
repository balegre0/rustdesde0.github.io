---
titulo: "Calcula el layout de repr(C)"
mundo: 21-el-compilador-por-dentro
tema: layout
dificultad: media
---

Con `#[repr(C)]` el layout de un struct es predecible: los campos van **en orden de declaración**, cada uno en el primer desplazamiento que sea múltiplo de su alineamiento, y el tamaño total se redondea hacia arriba a un múltiplo del alineamiento del struct (el mayor de sus campos).

Escribe `layout_c(campos: &[(usize, usize)]) -> (Vec<usize>, usize, usize)`: recibe `(tamaño, alineamiento)` de cada campo y devuelve `(desplazamientos, tamaño_total, alineamiento)`.

Ejemplo: `struct { a: u8, b: u32, c: u16 }` → `[(1, 1), (4, 4), (2, 2)]` → desplazamientos `[0, 4, 8]`, tamaño `12`, alineamiento `4`: el mismo 12 que verificamos con el compilador en la lección de layout. Un struct sin campos mide 0 y tiene alineamiento 1.

Todos los casos de prueba se han comprobado contra rustc con `size_of`, `align_of` y `offset_of!`.

<details><summary>Pista</summary>

Redondear `x` hacia arriba a un múltiplo de `a` (una potencia de dos): `(x + a - 1) / a * a`.

</details>
