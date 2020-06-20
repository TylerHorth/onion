use crate::onion::Onion;
use anyhow::Result;

mod ascii85;
mod onion;

fn main() -> Result<()> {
    let mut onion = Onion::new();

    onion.peel(Ok)?;
    onion.print();

    Ok(())
}
