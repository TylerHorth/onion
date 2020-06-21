use anyhow::Result;

use crate::bitwise::flip_and_rotate;
use crate::encryption::decrypt;
use crate::onion::Onion;
use crate::parity::checksum;

mod ascii85;
mod bitwise;
mod encryption;
mod onion;
mod parity;

fn main() -> Result<()> {
    let mut onion = Onion::new();

    onion.peel(Ok)?;
    onion.peel(flip_and_rotate)?;
    onion.peel(checksum)?;
    onion.peel(decrypt)?;
    onion.print();

    Ok(())
}
