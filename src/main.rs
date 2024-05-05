use std::process::Command;

fn main() {
    Command::new("./Game/Binaries/Win64/Game-Win64-Shipping.exe").spawn().unwrap().wait().unwrap();
}
