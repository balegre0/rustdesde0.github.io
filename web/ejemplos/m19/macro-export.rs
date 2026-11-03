mod interno {
    pub mod profundo {
        // 1. #[macro_export] la coloca en la RAÍZ de la crate
        //    y la hace visible para otras crates
        #[macro_export]
        macro_rules! version {
            () => { "1.0.0" };
        }
    }
}

fn main() {
    // 2. Se nombra como crate::version!, no como crate::interno::profundo::version!
    println!("{}", crate::version!());
    println!("{}", version!());
}
