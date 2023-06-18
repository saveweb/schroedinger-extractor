use std::fmt::{Display, Result as FmtResult};

#[derive(Debug)]
pub enum Vid {
    Avid(usize),
    Bvid(String),
}

impl Display for Vid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        match self {
            Vid::Avid(avid) => f.write_fmt(format_args!("av{}", avid)),
            Vid::Bvid(bvid) => f.write_fmt(format_args!("bv{}", bvid)),
        }
    }
}

pub fn try_into_vid(s: &str) -> Option<Vid> {
    let lower = s.to_lowercase();
    if lower.starts_with("av") {
        Some(Vid::Avid(
            lower[2..]
                .parse::<usize>()
                // .map_err(|e| e.to_string())
                .ok()?,
        ))
    } else if lower.starts_with("bv") {
        Some(Vid::Bvid(
            s[2..]
                .to_string() // Base58 不使用
                .replace("0", "o") // 数字"0"，
                .replace("O", "o") // 字母大写"O"，
                .replace("l", "1") // 和字母小写"l"
                .replace("I", "1"), // 字母大写"I"，
        ))
    } else {
        None
    }
}
