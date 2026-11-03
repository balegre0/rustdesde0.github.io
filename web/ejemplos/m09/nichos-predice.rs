use std::mem::size_of;

fn main() {
    println!("u32: {}  Option<u32>: {}", size_of::<u32>(), size_of::<Option<u32>>());
    println!("&u32: {}  Option<&u32>: {}", size_of::<&u32>(), size_of::<Option<&u32>>());
}
