fn main() {
    let mut i = 0;
    let x: i32 = loop {
        i += 1;
        if i == 3 {
            break;
        }
    };
    println!("{x}");
}
