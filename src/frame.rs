use chrono::NaiveDate;
use paddleocr::ContentData;
use std::fmt::{Display, Result as FmtResult};
use std::fs::canonicalize;
use std::path::PathBuf;
use std::time::Duration;

use crate::ocr::ocr::ocr_and_parse;

pub struct ShuukanFrame {
    pub path: PathBuf,
    pub time: Duration,
}

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

#[derive(Debug)]
pub struct ShuukanVideoInfo {
    pub vid: Vid,
    pub title: String,
    pub time: String,
}

impl ShuukanFrame {
    pub fn from(path: PathBuf, time: Duration) -> Result<ShuukanFrame, String> {
        if !path.exists() {
            return Err(format!("File not found: {:?}", path));
        }
        Ok(ShuukanFrame { path, time })
    }

    pub fn ocr(&self) -> Result<Vec<ContentData>, String> {
        ocr_and_parse(
            canonicalize(self.path.clone())
                .or_else(|e| Err(e.to_string()))?
                .to_str()
                .unwrap(),
        )
    }

    pub fn get_info(&self) -> Result<ShuukanVideoInfo, String> {
        let ocr_result = self.ocr()?;
        let is_single_video = ocr_result
            .iter()
            .filter(|x| x.text.contains("投稿"))
            .next()
            .is_none();
        if is_single_video {
            print!("<不是单个视频>")
        }
        let vid = ocr_result
            .iter()
            .filter_map(|x| {
                // lower x starts with av or bv
                let lower = x.text.to_lowercase();
                if lower.starts_with("av") {
                    Some(Vid::Avid(
                        lower[2..]
                            .parse::<usize>()
                            .map_err(|e| e.to_string())
                            .ok()?,
                    ))
                } else if lower.starts_with("bv") {
                    Some(Vid::Bvid(x.text[2..].to_string()))
                } else {
                    None
                }
            })
            .next()
            .ok_or("No vid found")?;
        let title = ocr_result
            .iter()
            .filter(|x| {
                let left = x.rect[0][0];
                let top = x.rect[0][1];
                let bottom = x.rect[3][1];
                left < 150 && top < 1001 && bottom > 1001
            })
            .next()
            .ok_or("No title found")?
            .text
            .to_string();
        let time = ocr_result
            .iter()
            .filter_map(|x| {
                // yyyy-mm-dd hh:mm
                NaiveDate::parse_from_str(&x.text, "%Y-%m-%d %H:%M").ok()
            })
            .next()
            .ok_or("No time found")?;

        Ok(ShuukanVideoInfo {
            vid,
            title,
            time: time.to_string(),
        })
    }
}
