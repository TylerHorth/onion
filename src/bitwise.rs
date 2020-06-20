use anyhow::Result;

const FLIP_MASK: u8 = 0b01010101;

pub fn flip_and_rotate(input: Vec<u8>) -> Result<Vec<u8>> {
    Ok(input
        .into_iter()
        .map(|b| b ^ FLIP_MASK)
        .map(|b| b.rotate_right(1))
        .collect())
}
