// Una biblioteca que no necesita sistema operativo: solo `core`.
#![no_std]

/// Suma de comprobación de un bloque de bytes (sin reservar memoria, sin imprimir).
pub fn suma_comprobacion(datos: &[u8]) -> u8 {
    let mut s: u8 = 0;
    for &b in datos {
        s = s.wrapping_add(b);
    }
    s
}

/// Formatea un número en un búfer propio: sin String ni heap.
pub fn a_texto(mut n: u32, buf: &mut [u8; 10]) -> &str {
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    core::str::from_utf8(&buf[i..]).unwrap_or("?")
}
