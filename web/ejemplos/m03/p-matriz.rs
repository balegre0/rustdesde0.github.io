use std::mem::size_of;

fn main() {
    let tablero = [[0u8; 3]; 2];
    println!("{}", tablero.len());
    println!("{}", tablero[0].len());
    println!("{}", size_of::<[[u8; 3]; 2]>());
    println!("{:?}", tablero);
}
