use std::mem::size_of;
use std::num::NonZeroU32;

#[allow(dead_code)]
enum Pequeno { A, B, C }

#[allow(dead_code)]
enum Forma {
    Circulo(f64),
    Rectangulo(f64, f64),
    Punto,
}

#[allow(dead_code)]
enum DosCasos { Caja(Box<u64>), Nada }

#[allow(dead_code)]
enum TresCasos { Caja(Box<u64>), Nada, Otra }

macro_rules! medir {
    ($t:ty) => {
        println!("{:>26} → {:>2} bytes", stringify!($t), size_of::<$t>());
    };
}

fn main() {
    println!("1. Sin datos: solo la etiqueta.");
    medir!(Pequeno);

    println!("2. Con datos: la variante más grande + etiqueta + relleno.");
    medir!(Forma);

    println!("3. Nicho de las referencias y Box: nunca valen 0.");
    medir!(&u8);
    medir!(Option<&u8>);
    medir!(Box<u8>);
    medir!(Option<Box<u8>>);
    medir!(DosCasos);
    medir!(TresCasos);

    println!("4. Los punteros crudos SÍ pueden valer 0: no hay nicho.");
    medir!(*const u8);
    medir!(Option<*const u8>);

    println!("5. Enteros: todos los valores son válidos… salvo en NonZero.");
    medir!(Option<u8>);
    medir!(Option<u32>);
    medir!(Option<NonZeroU32>);

    println!("6. bool y char tienen valores imposibles de sobra.");
    medir!(Option<bool>);
    medir!(Option<Option<bool>>);
    medir!(Option<char>);

    println!("7. String y Vec: el puntero nunca es nulo.");
    medir!(String);
    medir!(Option<String>);
}
