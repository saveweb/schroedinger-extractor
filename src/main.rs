mod bili;
mod frame;
mod ocr;
mod video;

use std::{path::PathBuf, time::Duration};
use video::ShuukanVideo;

const 开头时长: u64 = 5;
const 片尾时长: u64 = 200;
const 识别间隔: usize = 10;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let videos = ["C:/Users/Neko/Videos/周刊哔哩哔哩排行榜#599.mp4"];
    videos.into_iter().for_each(|v| process_video(v.into()));
}

fn process_video(video: PathBuf) {
    let video = ShuukanVideo::from(video).unwrap();
    println!("duration: {:?}", video.duration);
    // let frame_output_file: PathBuf = "C:/Users/Neko/Videos/frame302.jpg".into();
    // let frame = video
    //     .get_frame(Duration::from_secs(302), Some(frame_output_file))
    //     .unwrap();
    for i in (开头时长..video.duration.as_secs() - 片尾时长).step_by(识别间隔) {
        let frame = video.get_frame(Duration::from_secs(i), None).unwrap();
        print!("第 {} 秒：", i);
        let now = std::time::Instant::now();
        match frame.get_info() {
            Ok(info) => print!("{}", info),
            Err(e) => print!("未识别到信息，因为 {}", e),
        }
        println!("，耗时 {:?}", now.elapsed());
    }
}
