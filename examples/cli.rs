use rust_it8951::{It8951, Mode};
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("Start");
    let args: Vec<String> = env::args().collect();
    let width_pos: u32 = args[2].parse::<u32>().unwrap();
    let height_pos: u32 = args[3].parse::<u32>().unwrap();

    let mut it8951 = It8951::connect()?;

    thread::sleep(Duration::from_millis(100));
    println!("We are now reading data");
    let system_info = it8951.get_system_info().unwrap();
    println!("width: {}", system_info.width);
    println!("height: {}", system_info.height);
    println!("mode: {}", system_info.mode);
    println!("version: {}", system_info.version);

    println!("Display data from file {}", args[1]);
    let img = image::open(&args[1])?;
    let grayscale_image = img.grayscale();

    it8951.update_region(&grayscale_image, width_pos, height_pos, Mode::GC16)?;
    println!("End");
    Ok(())
}
