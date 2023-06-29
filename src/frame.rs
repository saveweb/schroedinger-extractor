use chrono::NaiveDate;
use paddleocr::ContentData;
use std::fmt::Display;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::bili::Vid;
use crate::ocr::ocr_and_parse;

pub struct ShuukanFrame {
    pub path: PathBuf,
    pub time: Duration,
}

pub struct ShuukanVideoInfo {
    pub vid: Vid,
    pub title: String,
    pub time: String,
}

impl Display for ShuukanVideoInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}] {} ({})",
            self.vid, self.title, self.time
        ))
    }
}

impl ShuukanFrame {
    pub fn from(path: PathBuf, time: Duration) -> Result<ShuukanFrame, String> {
        if !path.exists() {
            return Err(format!("File not found: {:?}", path));
        }
        Ok(ShuukanFrame { path, time })
    }

    pub fn ocr(&self) -> Result<Arc<[ContentData]>, String> {
        ocr_and_parse(
            canonicalize(self.path.clone())
                .or_else(|e| Err(e.to_string()))?
                .to_str()
                .unwrap(),
        )
        .and_then(|x| Ok(x.into()))
    }

    fn find_vid(ocr_result: &[ContentData]) -> Result<Vid, String> {
        ocr_result
            .iter()
            .find_map(|x| x.text.clone().try_into().ok())
            .ok_or("No vid found".into())
    }

    fn find_time(ocr_result: &[ContentData]) -> Result<NaiveDate, String> {
        ocr_result
            .iter()
            .find_map(|x| {
                // yyyy-mm-dd hh:mm
                NaiveDate::parse_from_str(&x.text, "%Y-%m-%d %H:%M").ok()
            })
            .ok_or("No time found".into())
    }

    fn find_title(ocr_result: &[ContentData]) -> Result<String, String> {
        Ok(ocr_result
            .iter()
            .filter(|x| {
                let left = x.rect[0][0];
                let top = x.rect[0][1];
                let bottom = x.rect[3][1];
                left < 150 && top < 1001 && bottom > 1001
            })
            .next()
            .ok_or("No title found, maybe bangumi")?
            .text
            .clone())
    }

    pub fn detect_is_single_video(ocr_result: &[ContentData]) -> bool {
        ocr_result
            .iter()
            .filter(|x| x.text.contains("投稿"))
            .next()
            .is_none()
    }

    pub fn get_info(&self) -> Result<ShuukanVideoInfo, String> {
        let ocr_result = self.ocr()?;
        if Self::detect_is_single_video(&ocr_result) {
            print!("<不是单个视频> ")
        }
        let vid = Self::find_vid(&ocr_result)?;
        let title = Self::find_title(&ocr_result)?;
        let time = Self::find_time(&ocr_result)?;

        Ok(ShuukanVideoInfo {
            vid,
            title,
            time: time.to_string(),
        })
    }
}
