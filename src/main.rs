use anyhow::Result;

use crate::bitwise::flip_and_rotate;
use crate::onion::Onion;
use crate::parity::checksum;

mod ascii85;
mod bitwise;
mod onion;
mod parity;

fn main() -> Result<()> {
    let mut onion = Onion::new();

    onion.peel(Ok)?;
    onion.peel(flip_and_rotate)?;
    onion.peel(checksum)?;
    onion.print();

    Ok(())
}
