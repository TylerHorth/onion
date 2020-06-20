use crate::ascii85::decode;

use anyhow::Result;

const ONION: &str = include_str!("onion.txt");

pub struct Onion {
    data: String,
}

impl Onion {
    pub fn new() -> Onion {
        Onion {
            data: ONION.to_string(),
        }
    }

    pub fn print(&self) {
        if let Some(payload_start) = self.data.find("<~") {
            let (description, rest) = self.data.split_at(payload_start + 2);
            let payload_end = rest.find("~>").unwrap();
            let (_payload, end) = rest.split_at(payload_end);

            print!("{}...{}", description, end);
        } else {
            print!("{}", self.data);
        }
    }

    pub fn peel(&mut self, peeler: impl Fn(Vec<u8>) -> Result<Vec<u8>>) -> Result<()> {
        let decoded = decode(&self.data)?;
        let peeled = peeler(decoded)?;
        self.data = String::from_utf8(peeled)?;

        Ok(())
    }
}
