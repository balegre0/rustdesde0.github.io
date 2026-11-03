fn main() {
    let s = String::from("ñandú");
    // los índices son de bytes: 'ñ' ocupa 2 bytes y 'ú' otros 2
    println!("{}", s.len());
    println!("{:?}", s.as_bytes());
    println!("{}", &s[0..2]);
    println!("{}", s.is_char_boundary(1));
    // de bytes a String: puede fallar si no es UTF-8 válido
    let bien = String::from_utf8(vec![104, 111, 108, 97]);
    let mal = String::from_utf8(vec![0xff, 0x61]);
    println!("{bien:?}");
    println!("{}", mal.is_err());
    println!("{}", String::from_utf8_lossy(&[0xff, 0x61]));
}
