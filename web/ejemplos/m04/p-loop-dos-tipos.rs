fn main() {
    let lleno = true;
    let v = loop {
        if lleno {
            break 1;
        }
        break "dos";
    };
    println!("{v}");
}
