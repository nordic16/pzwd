mod pzwd;

fn main() {
    if let Err(e) = pzwd::interactive() {
        eprintln!("Invalid input!!!");

        println!("{}", e);
    };
}