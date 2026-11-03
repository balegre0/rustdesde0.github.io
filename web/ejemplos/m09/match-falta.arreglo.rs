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
        Direccion::Oeste => 270,
    }
}

fn main() {
    for d in [Direccion::Norte, Direccion::Este, Direccion::Sur, Direccion::Oeste] {
        println!("{}", grados(d));
    }
}
