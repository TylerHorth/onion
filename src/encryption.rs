use anyhow::Result;

const KEY_LEN: usize = 32;
const NEEDLE: &[u8] = b"==[ Payload ]===============================================";

pub fn decrypt(input: Vec<u8>) -> Result<Vec<u8>> {
    // Assume space is the most common character. It is exceedingly unlikely that this will be the
    // decider as the needle is longer then the key, and thus we are able to double verify it.
    let mut max_spaces = 0;
    let mut max_spaces_idx = 0;

    'outer: for i in 0..=(input.len() - NEEDLE.len()) {
        let mut num_spaces = 0;

        for chunk in input.chunks_exact(KEY_LEN) {
            let mut predicted_chars = Vec::new();

            for j in 0..NEEDLE.len() {
                let index = i + j;
                let chunk_index = index % KEY_LEN;
                let prediction = input[index] ^ chunk[chunk_index] ^ NEEDLE[j];

                if predicted_chars.len() < KEY_LEN {
                    if prediction == b' ' {
                        num_spaces += 1;
                    }
                    predicted_chars.push(prediction);
                } else if prediction != predicted_chars[j % KEY_LEN] {
                    // contradiction, this cannot be the needle position
                    continue 'outer;
                }
            }
        }

        if num_spaces > max_spaces {
            max_spaces = num_spaces;
            max_spaces_idx = i;
        }
    }

    let mut key = [0; KEY_LEN];
    for i in 0..KEY_LEN {
        key[(max_spaces_idx + i) % KEY_LEN] = input[max_spaces_idx + i] ^ NEEDLE[i];
    }

    let mut result = Vec::new();
    for i in 0..input.len() {
        result.push(input[i] ^ key[i % KEY_LEN]);
    }

    Ok(result)
}
