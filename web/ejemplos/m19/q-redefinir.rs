macro_rules! version { () => { 1 }; }

fn antes() -> i32 { version!() }

macro_rules! version { () => { 2 }; }

fn despues() -> i32 { version!() }

fn main() {
    println!("{} {}", antes(), despues());
}
