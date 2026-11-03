use std::convert::TryFrom;

#[derive(Debug)]
struct Celsius(f64);
#[derive(Debug)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

#[derive(Debug)]
struct Porcentaje(u8);

impl TryFrom<i32> for Porcentaje {
    type Error = String;
    fn try_from(n: i32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&n) { Ok(Porcentaje(n as u8)) } else { Err(format!("{n} no es un porcentaje")) }
    }
}

fn main() {
    // From e Into: conversiones que no pueden fallar
    let f = Fahrenheit::from(Celsius(100.0));
    let g: Fahrenheit = Celsius(0.0).into(); // Into sale gratis de From
    println!("{f:?} {g:?} {}", f.0 - g.0);
    let s: String = String::from("hola");
    let n: i64 = i64::from(7u8);
    let v: Vec<u8> = "abc".into();
    println!("{s} {n} {v:?}");
    // TryFrom e TryInto: conversiones que pueden fallar
    println!("{:?}", Porcentaje::try_from(42));
    if let Ok(p) = Porcentaje::try_from(7) {
        println!("{}%", p.0);
    }
    println!("{:?}", Porcentaje::try_from(130));
    let r: Result<u8, _> = 300i32.try_into();
    println!("{r:?}");
    println!("{:?}", u8::try_from(200i32));
}
