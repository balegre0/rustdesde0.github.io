macro_rules! cuadrados {
    ($($x:expr),* $(,)?) => {
        vec![$( $x * $x ),*]
    };
}

fn main() {
    println!("{:?}", cuadrados![1, 2, 3]);
    println!("{:?}", cuadrados![4, 5,]);
}
