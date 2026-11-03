fn main() {
    let a = {
        let b = 2;
        let c = { b * 10 };
        c + b
    };
    println!("{a}");
}
