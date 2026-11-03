---
titulo: "Leer un u32 desalineado"
mundo: 20-unsafe-y-ub
tema: unsafe
dificultad: facil
---

Los formatos binarios guardan números en posiciones arbitrarias: un `u32` puede empezar en el byte 1,
que no es múltiplo de 4. Escribe `leer_u32_le(bytes, pos)`, que lee los 4 bytes que empiezan en `pos`
como un `u32` *little-endian* (el byte de menor peso primero).

- Si no hay 4 bytes a partir de `pos`, devuelve `None` (¡sin desbordar al sumar!).
- Lee con un puntero crudo y `read_unaligned`: `*p` exigiría alineación y aquí sería UB.
- Convierte con `u32::from_le` para que funcione en cualquier máquina.

**Ejemplo**

```text
bytes = [0xAA, 0x01, 0x00, 0x00, 0x00], pos = 1   →   Some(1)
bytes = [1, 2, 3], pos = 0                         →   None
```

<details><summary>Pista</summary>

`pos.checked_add(4)` evita el desbordamiento. El puntero: `bytes.as_ptr().add(pos).cast::<u32>()`.
</details>
