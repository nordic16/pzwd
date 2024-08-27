mod pzwd;
mod crypt;

fn main() -> anyhow::Result<()> {
    let val = pzwd::interactive()?;

    println!("{}", val);

    Ok(())
}
