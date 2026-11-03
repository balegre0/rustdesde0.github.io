use std::process::ExitCode;

fn main() -> ExitCode {
    let entrada = "abc";
    match entrada.parse::<i32>() {
        Ok(n) => {
            println!("número {n}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("no es un número: {e}");
            ExitCode::from(3)
        }
    }
}
