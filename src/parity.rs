use anyhow::Result;

pub fn checksum(input: Vec<u8>) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut written = 0;

    for byte in input {
        let masked = byte & !1;
        let num_ones = masked.count_ones() as u8;

        if num_ones & 1 != byte & 1 {
            // throw away byte
            continue;
        }

        let index = (written % 8) as u8;

        if index == 0 {
            result.push(masked);
        } else {
            let last = result.last_mut().unwrap();
            *last |= masked >> (8 - index);
            if index < 7 {
                result.push(masked << index);
            }
        }

        written += 1;
    }

    Ok(result)
}
