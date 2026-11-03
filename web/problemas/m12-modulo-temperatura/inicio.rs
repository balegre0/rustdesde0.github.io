mod temperatura {
    // Completa: struct Celsius (campo privado), nueva, grados y el submódulo conversion.
    pub struct Celsius {
        grados: f64,
    }

    impl Celsius {
        pub fn nueva(g: f64) -> Option<Celsius> {
            todo!()
        }

        pub fn grados(&self) -> f64 {
            todo!()
        }
    }

    pub mod conversion {
        pub fn a_fahrenheit(c: &super::Celsius) -> f64 {
            todo!()
        }
    }
}
