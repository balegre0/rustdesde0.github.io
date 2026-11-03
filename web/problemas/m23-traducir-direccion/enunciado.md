---
titulo: "Traducir una dirección virtual"
mundo: 23-embebido-y-tu-so
tema: memoria
dificultad: dificil
---

Simula la paginación de **dos niveles** de un procesador de juguete con direcciones de 16 bits y páginas de 256 bytes:

```text
 15     12 11      8 7              0
┌─────────┬─────────┬────────────────┐
│ nivel 1 │ nivel 2 │  desplazamiento│
└─────────┴─────────┴────────────────┘
```

`tabla1` tiene 16 entradas; cada una es `Some(i)` (índice de una tabla de segundo nivel en `tablas2`) o `None`. Cada tabla de segundo nivel tiene 16 entradas: `Some(marco)` (número de marco físico) o `None`.

Escribe `traducir(virt: u16, tabla1: &[Option<usize>; 16], tablas2: &[[Option<u16>; 16]]) -> Result<u32, Fallo>`, que devuelve la dirección física `marco * 256 + desplazamiento`, o `Err(Fallo::Nivel1)` / `Err(Fallo::Nivel2)` según en qué nivel falte la entrada (un fallo de página).
