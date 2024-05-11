use anyhow::Result;
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut pwd = Vec::new();
    let mut chars = Vec::new();
    if uppercase {
        chars.extend_from_slice(UPPER);
        pwd.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        pwd.push(
            *LOWER
                .choose(&mut rng)
                .expect("LOWER won't be empty in this context"),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        pwd.push(
            *NUMBER
                .choose(&mut rng)
                .expect("NUMBER won't be empty in this context"),
        );
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        pwd.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("SYMBOL won't be empty in this context"),
        );
    }
    for _ in 0..(length - pwd.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        pwd.push(*c);
    }

    pwd.shuffle(&mut rng);
    let password = String::from_utf8(pwd)?;
    Ok(password)
}
