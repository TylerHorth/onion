use std::convert::TryInto;

use anyhow::{bail, Context, Result};

const RADIX: u32 = 85;
const ENCODE_CHUNK_SIZE: usize = 4;
const DECODE_GROUP_SIZE: usize = 5;
const START_CHAR: u8 = b'!';
const END_CHAR: u8 = b'u';
const ZERO_CHAR: u8 = b'z';
const START_DELIMITER: &str = "<~";
const END_DELIMITER: &str = "~>";

#[allow(dead_code)]
pub fn encode(input: &[u8]) -> Result<String> {
    let mut result = String::new();
    let chunks = input.chunks_exact(ENCODE_CHUNK_SIZE);

    let remainder = chunks.remainder();
    let remainder_encoded = if !remainder.is_empty() {
        let mut chunk = [0u8; ENCODE_CHUNK_SIZE];
        chunk[..remainder.len()].copy_from_slice(remainder);
        Some(u32::from_be_bytes(chunk))
    } else {
        None
    };

    result.push_str(START_DELIMITER);

    let mut buffer = ['\0'; DECODE_GROUP_SIZE];
    for mut value in chunks
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .chain(remainder_encoded)
    {
        for i in 0..DECODE_GROUP_SIZE {
            buffer[i] = ((value % RADIX) as u8 + START_CHAR) as char;
            value /= RADIX;
        }

        result.extend(buffer.iter().rev());
    }

    if !remainder.is_empty() {
        result.truncate(result.len() - (ENCODE_CHUNK_SIZE - remainder.len()));
    }
    result.push_str(END_DELIMITER);

    Ok(result)
}

pub fn decode(input: &str) -> Result<Vec<u8>> {
    let start = input
        .find(START_DELIMITER)
        .context("missing start delimiter")?
        + START_DELIMITER.len();
    let input = input.split_at(start).1;

    let end = input.find(END_DELIMITER).context("missing end delimiter")?;
    let input = input.split_at(end).0;

    let mut group = CharGroup::new();
    let mut output = Vec::new();

    for byte in input.bytes() {
        match byte {
            b'z' if group.is_empty() => {
                output.extend_from_slice(&[0; ENCODE_CHUNK_SIZE]);
            }
            b'z' => {
                bail!("Misaligned '{}' character", ZERO_CHAR as char);
            }
            b'!'..=b'u' => {
                if let Some(data) = group.accumulate(byte) {
                    output.extend_from_slice(&data);
                }
            }
            c if c.is_ascii_whitespace() => {
                // Ignore whitespace
            }
            c => {
                bail!("Character '{}' outside range", c as char);
            }
        }
    }

    if !group.is_empty() {
        let mut keep = ENCODE_CHUNK_SIZE;
        loop {
            keep -= 1;
            if let Some(data) = group.accumulate(END_CHAR) {
                output.extend_from_slice(&data[..keep]);
                break;
            }
        }
    }

    Ok(output)
}

struct CharGroup {
    index: usize,
    value: u32,
}

impl CharGroup {
    pub fn new() -> CharGroup {
        CharGroup { index: 0, value: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub fn accumulate(&mut self, byte: u8) -> Option<[u8; ENCODE_CHUNK_SIZE]> {
        self.value *= RADIX;
        self.value += (byte - START_CHAR) as u32;

        if self.index == DECODE_GROUP_SIZE - 1 {
            let bytes = self.value.to_be_bytes();
            self.index = 0;
            self.value = 0;
            Some(bytes)
        } else {
            self.index += 1;
            None
        }
    }
}

#[cfg(test)]
mod test {
    use rand::RngCore;

    use super::*;

    const TEXT: &str =
        "Man is distinguished, not only by his reason, but by this singular passion \
         from other animals, which is a lust of the mind, that by a perseverance of \
         delight in the continued and indefatigable generation of knowledge, exceeds \
         the short vehemence of any carnal pleasure.";

    const DATA: &str =
        "<~9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,\
         O<DJ+*.@<*K0@<6L(Df-\\0Ec5e;DffZ(EZee.Bl.9pF\"AGXBPCsi+DGm>@3BB/F*&OCAfu2/A\
         KYi(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolD\
         Ial(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG\
         %G>uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c~>";

    #[test]
    fn test_encode() {
        assert_eq!(encode(TEXT.as_bytes()).unwrap(), DATA);
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(DATA).unwrap(), TEXT.as_bytes());
    }

    #[test]
    fn test_stochastic() {
        let mut rng = rand::thread_rng();

        for len in 0..500 {
            for _ in 0..5 {
                let mut input = vec![0; len];
                rng.fill_bytes(&mut input);

                let encoded = encode(&input).unwrap();
                let decoded = decode(&encoded).unwrap();

                assert_eq!(input, decoded);
            }
        }
    }
}
