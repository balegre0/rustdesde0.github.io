fn main() {
    let x = 'b: {
        if true {
            break 5;
        }
        0
    };
    println!("{x}");
}
