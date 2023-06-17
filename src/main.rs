use std::{path::PathBuf, time::Duration};
use video::ShuukanVideo;
mod frame;
mod ocr;
mod video;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    let video: PathBuf = "C:/Users/Neko/Videos/周刊哔哩哔哩排行榜#599.mp4".into();
    let video = ShuukanVideo::from(video).unwrap();
    println!("duration: {:?}", video.duration);
    // let frame_output_file: PathBuf = "C:/Users/Neko/Videos/frame302.jpg".into();
    // let frame = video
    //     .get_frame(Duration::from_secs(302), Some(frame_output_file))
    //     .unwrap();
    for i in (15..video.duration.as_secs() - 200).step_by(10) {
        let frame = video.get_frame(Duration::from_secs(i), None).unwrap();
        print!("第 {} 秒：", i);
        let now = std::time::Instant::now();
        match frame.get_info() {
            Ok(info) => print!("{:?}", info),
            Err(e) => print!("未识别到信息，因为 {}", e),
        }
        println!("，耗时 {:?}", now.elapsed());
    }
}
