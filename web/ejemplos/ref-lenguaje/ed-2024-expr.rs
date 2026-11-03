// Edición 2024: el fragmento `expr` también encaja con `const { … }`.
// En 2021 ganaba la segunda regla; ahora gana la primera.
macro_rules! cual {
    ($e:expr) => { "primera regla (expr)" };
    (const $b:block) => { "segunda regla (const)" };
}
fn main() { println!("{}", cual!(const { 1 + 1 })); }
