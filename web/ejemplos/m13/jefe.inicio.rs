use std::fmt;
use std::ops::Add;

// 1. Deriva lo necesario para Area (Debug, Clone, Copy, PartialEq, PartialOrd, Default).
struct Area(f64);

// 2. Implementa Add para Area.

// 3. Implementa Display para Area: dos decimales y " m²" (p. ej. "4.00 m²").

trait Figura {
    fn nombre(&self) -> String;
    fn area(&self) -> Area;
    // 4. Añade un método por defecto `ficha` que devuelva
    //    format!("{:<10} {}", self.nombre(), self.area())
}

struct Cuadrado {
    lado: f64,
}
struct Circulo {
    radio: f64,
}

// 5. Implementa Figura para Cuadrado ("cuadrado") y Circulo ("círculo", área 3.14159·r²).

// 6. Implementa From<f64> para Cuadrado.

// 7. Escribe `total`, genérica: suma un &[T] partiendo de T::default().
//    Bounds: Add<Output = T> + Default + Copy.

fn main() {
    println!("Descomenta el programa cuando todo compile.");
    // 8. Cuando todo compile, descomenta el programa:
    // let figuras: Vec<Box<dyn Figura>> = vec![
    //     Box::new(Cuadrado::from(2.0)),
    //     Box::new(Circulo { radio: 1.0 }),
    //     Box::new(Cuadrado { lado: 0.5 }),
    // ];
    // for f in &figuras {
    //     println!("{}", f.ficha());
    // }
    // let areas: Vec<Area> = figuras.iter().map(|f| f.area()).collect();
    // println!("Total: {}", total(&areas));
    // println!("Total de lados: {}", total(&[4, 0, 4]));
    // let mayor = areas.iter().copied().fold(Area::default(), |a, b| if b > a { b } else { a });
    // println!("Mayor: {mayor}");
}
