use anyhow::Result;

use crate::aes::aes_cbc;
use crate::bitwise::flip_and_rotate;
use crate::encryption::decrypt;
use crate::network::udp;
use crate::onion::Onion;
use crate::parity::checksum;

mod aes;
mod ascii85;
mod bitwise;
mod encryption;
mod network;
mod onion;
mod parity;

fn main() -> Result<()> {
    let mut onion = Onion::new();

    onion.peel(Ok)?;
    onion.peel(flip_and_rotate)?;
    onion.peel(checksum)?;
    onion.peel(decrypt)?;
    onion.peel(udp)?;
    onion.peel(aes_cbc)?;

    onion.print();

    Ok(())
}
