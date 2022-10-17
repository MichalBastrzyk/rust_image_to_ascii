use clap::Parser;
use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView, Rgba};
use terminal_size::{terminal_size, Height, Width};

const CHARACTER_LIST: &str = "_.,-=+:;cba!?0123456789$w#@N";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file_path: String,
    #[arg(long = "img_w")]
    width: Option<u32>,
    #[arg(long = "img_h")]
    height: Option<u32>,
    #[arg(short, long)]
    inverted: Option<bool>,
}

fn main() {
    let args = Args::parse();

    // let args: Vec<String> = env::args().collect();
    // if args.len() <= 1 {
    //     panic!("Filepath is required")
    // }

    let size = terminal_size();

    if let Some((Width(w), Height(h))) = size {
        let img = ImageReader::open(&args.file_path).unwrap();
        let decoded_img = img.decode().unwrap();
        let resized_img;
        if args.width.is_none() && args.height.is_none() {
            resized_img = decoded_img.resize_exact(w.into(), h.into(), FilterType::Triangle);
            println!("Using default width and height");
        } else {
            println!("Using custom width and height");
            resized_img = decoded_img.resize_exact(
                args.width.unwrap().into(),
                args.height.unwrap().into(),
                FilterType::Triangle,
            );
        }

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
