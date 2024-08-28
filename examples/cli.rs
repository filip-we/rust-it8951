use rust_it8951::{It8951, Mode};
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("Start");
    let args: Vec<String> = env::args().collect();
    let file: &String = &args[1];
    let width_pos: u32 = args[2].parse::<u32>().unwrap();
    let height_pos: u32 = args[3].parse::<u32>().unwrap();
    let mode_input: u32 = args[4].parse::<u32>().unwrap();

    let mode = match mode_input {
        0 => Mode::INIT,
        1 => Mode::DU,
        2 => Mode::GC16,
        3 => Mode::GL16,
        4 => Mode::GLR16,
        5 => Mode::GLD16,
        6 => Mode::DU4,
        7 => Mode::A2,
        _ => Mode::__UNKNOWN1,
    };

    let mut it8951 = It8951::connect()?;

    thread::sleep(Duration::from_millis(100));
    println!("We are now reading data");
    let system_info = it8951.get_system_info().unwrap();
    println!("width: {}", system_info.width);
    println!("height: {}", system_info.height);
    println!("mode: {}", system_info.mode);
    println!("version: {}", system_info.version);

    println!("Displaying data from file {}", file);
    let img = image::open(file)?;
    let grayscale_image = img.grayscale();

    it8951.update_region(&grayscale_image, width_pos, height_pos, mode)?;
    println!("End");
    Ok(())
}
