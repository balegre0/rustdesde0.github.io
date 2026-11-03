fn main() {
    let cangrejo = "🦀";
    println!("{}", cangrejo.len());
    println!("{}", '🦀'.len_utf8());
    println!("{}", std::mem::size_of_val(&'🦀'));
    println!("{}", "€".len());
    println!("{}", cangrejo.chars().count());
}
