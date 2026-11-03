fn main() {
    let s = "ñu";
    // Pide el primer carácter (puede ocupar varios bytes), o el primer byte
    let primera = s.chars().next();
    let primer_byte = s.as_bytes()[0];
    println!("{primera:?}");
    println!("{primer_byte:#x}");
}
