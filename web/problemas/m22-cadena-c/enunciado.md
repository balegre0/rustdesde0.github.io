---
titulo: "Preparar texto para C"
mundo: 22-sistemas-y-ffi
tema: ffi
dificultad: media
---

Las funciones escritas en C reciben el texto como bytes terminados en un 0. Escribe `a_bytes_c(s: &str) -> Result<Vec<u8>, usize>` que devuelva los bytes de `s` seguidos de un 0 final.

Si `s` contiene un byte 0 en medio, no se puede representar (C creería que el texto termina ahí): devuelve `Err(posicion)` con la posición en bytes del primer 0.

Hazlo con `std::ffi::CString` (`CString::new`, `as_bytes_with_nul` y el error `NulError::nul_position`).
