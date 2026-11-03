// Una `expr` solo puede ir seguida de `=>`, `,` o `;`.
macro_rules! mal {
    ($a:expr + $b:expr) => { $a + $b };
}

fn main() {
    println!("{}", mal!(1 + 2));
}
