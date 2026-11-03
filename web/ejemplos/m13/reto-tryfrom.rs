#[derive(Debug)]
enum Dia {
    Lunes,
    Martes,
    Miercoles,
}

impl TryFrom<u8> for Dia {
    type Error = u8;
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(Dia::Lunes),
            2 => Ok(Dia::Martes),
            3 => Ok(Dia::Miercoles),
            otro => Err(otro),
        }
    }
}

fn main() {
    println!("{:?}", Dia::try_from(2));
    println!("{:?}", Dia::try_from(9));
}
