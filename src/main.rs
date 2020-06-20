use crate::bitwise::flip_and_rotate;
use crate::onion::Onion;
use anyhow::Result;

mod ascii85;
mod bitwise;
mod onion;

fn main() -> Result<()> {
    let mut onion = Onion::new();

    onion.peel(Ok)?;
    onion.peel(flip_and_rotate)?;
    onion.print();

    Ok(())
}
