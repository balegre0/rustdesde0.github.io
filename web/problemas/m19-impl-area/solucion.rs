pub trait Area {
    fn area(&self) -> f64 { 0.0 }
}

macro_rules! impl_area {
    ($($tipo:ty, $s:ident => $formula:expr);* $(;)?) => {
        $(
            impl Area for $tipo {
                fn area(&self) -> f64 {
                    let $s = self;
                    $formula
                }
            }
        )*
    };
}
