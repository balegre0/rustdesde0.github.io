fn area(lado: u32) -> u32 {
    lado * lado
}

fn main() {
    let lado: u8 = 12;
    // typeck compara el tipo del argumento con el de la firma: u8 no es u32.
    println!("{}", area(lado));
}
