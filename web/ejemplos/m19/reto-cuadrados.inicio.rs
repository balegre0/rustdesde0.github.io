// cuadrados![1, 2, 3] debe dar el Vec [1, 4, 9]. Acepta coma final.
macro_rules! cuadrados {
    ($($x:expr),* $(,)?) => { Vec::<i32>::new() };
}

fn main() {
    println!("{:?}", cuadrados![1, 2, 3]);
    println!("{:?}", cuadrados![4, 5,]);
}
