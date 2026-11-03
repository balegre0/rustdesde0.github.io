mod temperatura {
    pub struct Celsius {
        grados: f64,
    }

    impl Celsius {
        pub fn nueva(g: f64) -> Option<Celsius> {
            if g < -273.15 { None } else { Some(Celsius { grados: g }) }
        }

        pub fn grados(&self) -> f64 {
            self.grados
        }
    }

    pub mod conversion {
        use super::Celsius;

        pub fn a_fahrenheit(c: &Celsius) -> f64 {
            c.grados * 9.0 / 5.0 + 32.0
        }
    }
}
