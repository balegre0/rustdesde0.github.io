// La misma lógica no_std, probada en un programa normal: core está también dentro de std.
fn suma_comprobacion(datos: &[u8]) -> u8 {
    let mut s: u8 = 0;
    for &b in datos {
        s = s.wrapping_add(b);
    }
    s
}

fn a_texto(mut n: u32, buf: &mut [u8; 10]) -> &str {
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

fn main() {
    println!("{}", suma_comprobacion(b"hola"));
    let mut buf = [0u8; 10];
    println!("«{}»", a_texto(4096, &mut buf));
    println!("«{}»", a_texto(0, &mut buf));
}
