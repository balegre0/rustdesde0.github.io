fn main() {
    let r = ["a", "b", "c"].iter().fold(String::from(">"), |acc, s| acc + s);
    println!("{r}");
}
