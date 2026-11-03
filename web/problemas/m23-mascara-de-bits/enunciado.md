---
titulo: "Registros y máscaras"
mundo: 23-embebido-y-tu-so
tema: registros
dificultad: facil
---

Los registros de un periférico se manejan bit a bit. Escribe estas funciones sobre un valor `u32`:

- `poner(reg: u32, bit: u32) -> u32`: pone a 1 el bit indicado.
- `quitar(reg: u32, bit: u32) -> u32`: lo pone a 0.
- `alternar(reg: u32, bit: u32) -> u32`: lo invierte.
- `leer(reg: u32, bit: u32) -> bool`: dice si está a 1.
- `campo(reg: u32, desde: u32, ancho: u32) -> u32`: extrae `ancho` bits a partir del bit `desde` (por ejemplo, `campo(0b1101_0000, 4, 4) == 0b1101`). Supón `ancho` entre 1 y 31.

En todas, los demás bits no cambian.
