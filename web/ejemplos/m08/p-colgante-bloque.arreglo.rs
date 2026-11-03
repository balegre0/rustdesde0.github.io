fn main() {
    let x = 5;
    let r;
    {
        r = &x;
    }
    println!("{r}");
}
