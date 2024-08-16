mod pzwd;

fn main() {
    if let Err(_) = pzwd::interactive() {
        eprintln!("Invalid input!!!");
    };
}