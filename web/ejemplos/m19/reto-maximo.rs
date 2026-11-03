macro_rules! max {
    ($a:expr) => { $a };
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

fn main() {
    println!("{}", max!(7));
    println!("{}", max!(3, 9));
    println!("{}", max!(10 - 1, 2 * 2));
}
