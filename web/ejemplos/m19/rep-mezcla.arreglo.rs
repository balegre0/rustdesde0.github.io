macro_rules! parejas {
    ($($a:expr),* ; $($b:expr),*) => {
        [$( ($a, $b) ),*]
    };
}

fn main() {
    let p = parejas!(1, 2, 3 ; 'x', 'y', 'z');
    println!("{:?}", p);
}
