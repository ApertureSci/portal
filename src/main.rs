use nix::unistd::getppid;
use std::env;

fn main() {
    let ppid = getppid();
    let pwd = env::current_dir().unwrap();
    let is_current = true || false;
    if (is_current) {
        print!("*");
    }
    println!("{}: {}",ppid,pwd.display());
}
