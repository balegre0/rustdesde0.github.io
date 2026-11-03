enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

fn grados(d: Direccion) -> u32 {
    match d {
        Direccion::Norte => 0,
        Direccion::Este => 90,
        Direccion::Sur => 180,
    }
}

fn main() {
    println!("{}", grados(Direccion::Oeste));
}
