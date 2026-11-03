---
titulo: "Configuración según el destino"
mundo: 12-modulos-crates-y-cargo
tema: perfiles
dificultad: facil
---

Escribe `descripcion_destino() -> String` que use `cfg!` para describir la plataforma para la que se **compiló** el programa, con este formato exacto:

`"{arquitectura}, {bits} bits, {familia}"`

- `{arquitectura}`: `std::env::consts::ARCH`.
- `{bits}`: el número de bits de un puntero (`usize::BITS`).
- `{familia}`: `"wasm"` si `cfg!(target_family = "wasm")`, `"unix"` si `cfg!(unix)`, `"windows"` si `cfg!(windows)` y `"otra"` en cualquier otro caso (en ese orden).

Las pruebas se ejecutan en el navegador, donde el resultado es `"wasm32, 32 bits, wasm"`.
