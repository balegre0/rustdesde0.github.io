fn main() {
    let r;
    let x = 5; // ahora `x` vive tanto como `r`
    r = &x;
    println!("{r}");
}
