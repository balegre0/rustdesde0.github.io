---
titulo: "La máquina de estados del ascensor"
mundo: 09-structs-enums-y-patrones
tema: enums
dificultad: dificil
---

Un ascensor de un edificio con plantas 0 a 10 tiene tres estados:

```rust
enum Estado {
    Parado { planta: u8 },
    Subiendo { desde: u8, hasta: u8 },
    Bajando { desde: u8, hasta: u8 },
}
```

y recibe eventos:

```rust
enum Evento {
    Llamar(u8),   // alguien pulsa una planta
    Llegar,       // el ascensor llega a su destino
    Emergencia,   // freno de emergencia
}
```

Escribe `transicion(estado: Estado, evento: Evento) -> Estado` con estas reglas:

1. `Parado { planta }` + `Llamar(p)`: si `p > 10`, o `p == planta`, sigue igual. Si `p > planta`, pasa a `Subiendo { desde: planta, hasta: p }`; si `p < planta`, a `Bajando { … }`.
2. `Subiendo`/`Bajando` + `Llegar` → `Parado { planta: hasta }`.
3. `Subiendo`/`Bajando` + `Llamar(_)` → se ignora (sigue igual).
4. `Parado` + `Llegar` → sigue igual.
5. Cualquier estado + `Emergencia` → `Parado` en la planta de **origen** del movimiento (`desde`), o en la misma planta si ya estaba parado.

<details>
<summary>Pista 1</summary>

Haz `match (estado, evento)`: una tupla de dos enums. Los brazos más concretos (por ejemplo, la emergencia) pueden ir primero.
</details>

<details>
<summary>Pista 2</summary>

Para la regla 1 sirve una guarda: `(Estado::Parado { planta }, Evento::Llamar(p)) if p > planta && p <= 10 => …`. Y para «sigue igual», un brazo final `(e, _) => e` devuelve el estado que entró.
</details>
