use once_cell::sync::OnceCell;
use paddleocr::{ContentData, Ppocr};
use std::{path::PathBuf, sync::Mutex};

fn global_ppocr() -> &'static Mutex<Ppocr> {
    static INSTANCE: OnceCell<Mutex<Ppocr>> = OnceCell::new();
    let path: PathBuf = "E:/code/paddleocr/pojnew/PaddleOCR_json.exe".into();
    INSTANCE.get_or_init(|| Mutex::new(Ppocr::new(path).unwrap()))
}

pub fn ocr_and_parse(image: &str) -> Result<Vec<ContentData>, String> {
    let mut p = global_ppocr().lock().unwrap();
    let res = p.ocr_and_parse(image)?;
    Ok(res)
}
