fn main() {
    let o: Option<i32> = Some(3);
    let Some(x) = o else {
        println!("no había valor");
        return;
    };
    println!("{x}");
}
