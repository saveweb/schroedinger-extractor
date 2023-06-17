use std::{path::PathBuf, process::Command, time::Duration};

use crate::frame::ShuukanFrame;

pub struct ShuukanVideo {
    path: PathBuf,
    pub duration: Duration,
}

impl ShuukanVideo {
    pub fn from(path: PathBuf) -> Result<ShuukanVideo, String> {
        let duration = {
            let output = Command::new("ffprobe")
                // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 -sexagesimal input_file
                .args(&[
                    "-v",
                    "error",
                    "-show_entries",
                    "format=duration",
                    "-of",
                    "default=noprint_wrappers=1:nokey=1",
                    "-sexagesimal",
                    path.to_str().ok_or("Invalid path")?,
                ])
                .output()
                .map_err(|e| e.to_string())?;
            let output = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
            // parse
            // 0:24:18.000000
            let seconds = output
                .trim()
                .split(':')
                .map(|s| s.parse::<f64>())
                .collect::<Result<Vec<_>, _>>()
                .or(Err(format!("Invalid duration: {}", output)))?;
            let seconds = (seconds[0] * 3600.0 + seconds[1] * 60.0 + seconds[2]) * 1000.;
            Duration::from_millis(seconds as u64)
        };

        Ok(ShuukanVideo { path, duration })
    }

    pub fn get_frame(
        &self,
        time: Duration,
        image: Option<PathBuf>,
    ) -> Result<ShuukanFrame, String> {
        let image = image.unwrap_or_else(|| {
            let mut image = PathBuf::from("./images/");
            image.push(format!("frame-{}.jpg", time.as_secs()));
            image
        });
        if image.exists() {
            return Ok(ShuukanFrame::from(image, time).unwrap());
        }
        let output = Command::new("ffmpeg")
            // "ffmpeg -ss {time} -i input_file -vframes 1 output_file.jpg"
            .args(&[
                "-ss",
                &format!("{}", time.as_secs_f64()),
                "-i",
                self.path.to_str().unwrap(),
                "-vframes",
                "1",
                image.to_str().unwrap(),
            ])
            .output()
            .or(Err("Failed to execute ffmpeg"))?;
        if !output.status.success() {
            return Err(format!(
                "ffmpeg failed: {}",
                String::from_utf8(output.stderr).unwrap()
            ));
        }
        ShuukanFrame::from(image, time)
    }
}
