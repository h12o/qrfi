use rand::Rng;
use rand::seq::SliceRandom;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CharType {
    DoubleByte,
    TripleByte,
    QuadrupleByte,
}
use CharType::*;

pub fn generate_random_hex(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: &[u8] = b"0123456789abcdef";
    (0..len)
        .map(|_| chars[rng.gen_range(0..chars.len())] as char)
        .collect()
}

pub fn generate_random_ascii(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            rng.gen_range(0x20..=0x7E) as u8 as char
        })
        .collect()
}

pub fn generate_random_mbstring(n: usize, types: &[CharType]) -> String {
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    let mut byte = 0;
    let doublebyte_chars = "éßπç";
    let triplebyte_chars = "あいうえお、カキクケコ。ざじずぜぞ・ダヂヅデド×ﾅﾆﾇﾈﾉ÷ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ〄一二三四五々六七八九十〇「가각간갇갈」『감갑갖갗갉』";
    let quadruplebyte_chars = "🦀🚀🔥";
    let singlebyte_chars = "0123456789ABCDEFGHIJKLMNPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    for t in types {
        let pool = match t {
            DoubleByte => doublebyte_chars,
            TripleByte => triplebyte_chars,
            QuadrupleByte => quadruplebyte_chars,
        };
        let c = pool.chars().collect::<Vec<char>>().choose(&mut rng).unwrap().to_owned();
        let len = c.len_utf8();
        if byte + len <= n {
            chars.push(c);
            byte += len;
        }
    }
    let single_pool: Vec<char> = singlebyte_chars.chars().collect();
    while byte < n {
        let c = *single_pool.choose(&mut rng).unwrap();
        chars.push(c);
        byte += 1;
    }
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}
