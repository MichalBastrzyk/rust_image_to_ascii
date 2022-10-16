use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView, Rgba};
use std::env;
use terminal_size::{terminal_size, Height, Width};

const CHARACTER_LIST: &str = "_.,-=+:;cba!?0123456789$w#@N";
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("Filepath is required")
    }

    let size = terminal_size();

    if let Some((Width(w), Height(h))) = size {
        let img = ImageReader::open(&args[1]).unwrap();
        let decoded_img = img.decode().unwrap();
        let resized_img = decoded_img.resize_exact(w.into(), h.into(), FilterType::Triangle);

        for pixel in resized_img.pixels() {
            let rgba: Rgba<u8> = pixel.2;
            let average: u16 = rgba[0] as u16 + rgba[1] as u16 + rgba[2] as u16;
            let character_index = average as usize / CHARACTER_LIST.len();

            print!("{}", CHARACTER_LIST.chars().nth(character_index).unwrap());

            if pixel.0 == resized_img.width() {
                print!("\n")
            }
        }
    } else {
        println!("Unable to get terminal size");
    }
}
