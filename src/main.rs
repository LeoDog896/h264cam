use anyhow::Result;
use std::{io::Write, path::Path, fs::File};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the video device
    #[clap(short, long, default_value = "/dev/video0")]
    device_path: String,

    /// Maximum frames per second
    #[clap(short, long, default_value = "60")]
    max_fps: u32,

    /// Output file
    #[clap(short, long, default_value = "./output.h264")]
    output_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let device_path = Path::new(args.device_path.as_str());
    let max_fps = args.max_fps;

    let mut device = h264_webcam_stream::get_device(&device_path)?;
    let mut stream = h264_webcam_stream::stream(&mut device, max_fps)?;

    let mut f = File::create(args.output_file)?;

    for _ in 0..2 {
        let (h264_bytes, _) = stream.next(false)?;
        // Record the h264 video to a file
        f.write_all(&h264_bytes[..])?;
    }

    Ok(())
}