fn main() {
    let a: u8 = 0b1100_1010;
    let b: u8 = 0b1010_0110;
    println!("a      = {a:08b}");
    println!("b      = {b:08b}");
    println!("a & b  = {:08b}  (y: 1 si los dos son 1)", a & b);
    println!("a | b  = {:08b}  (o: 1 si alguno es 1)", a | b);
    println!("a ^ b  = {:08b}  (o exclusivo: 1 si son distintos)", a ^ b);
    println!("!a     = {:08b}  (no: invierte todos)", !a);
    println!("a << 2 = {:08b}  (desplaza a la izquierda)", a << 2);
    println!("a >> 3 = {:08b}  (desplaza a la derecha)", a >> 3);
    let mut x = 10;
    x += 5;
    x *= 2;
    x -= 1;
    x %= 7;
    println!("x = {x}");
}
