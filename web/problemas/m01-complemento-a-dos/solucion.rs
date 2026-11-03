fn con_signo(bits: &str) -> i8 {
    let mut total: i32 = 0;
    for (i, c) in bits.chars().enumerate() {
        if c == '1' {
            let peso = 1i32 << (7 - i);
            if i == 0 { total -= peso } else { total += peso }
        }
    }
    total as i8
}
