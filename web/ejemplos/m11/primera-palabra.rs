fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    println!("[{}]", primera_palabra("   árbol frondoso"));
    println!("[{}]", primera_palabra("ñu"));
    println!("[{}]", primera_palabra("   "));
}
