use crossterm::terminal::size;
use image::{imageops::FilterType::Lanczos3, DynamicImage, ImageReader};
use std::env;

const BRAILLE: [&str; 64] = [
    "⠀", "⠁", "⠂", "⠃", "⠄", "⠅", "⠆", "⠇", "⠈", "⠉", "⠊", "⠋", "⠌", "⠍", "⠎", "⠏", "⠐", "⠑", "⠒",
    "⠓", "⠔", "⠕", "⠖", "⠗", "⠘", "⠙", "⠚", "⠛", "⠜", "⠝", "⠞", "⠟", "⠠", "⠡", "⠢", "⠣", "⠤", "⠥",
    "⠦", "⠧", "⠨", "⠩", "⠪", "⠫", "⠬", "⠭", "⠮", "⠯", "⠰", "⠱", "⠲", "⠳", "⠴", "⠵", "⠶", "⠷", "⠸",
    "⠹", "⠺", "⠻", "⠼", "⠽", "⠾", "⠿",
];

fn make_braille_pic(picture: DynamicImage) -> Vec<Vec<u8>> {
    let num_cells = get_cells_size(picture.width(), picture.height());
    let braille_size = (num_cells.0 * 2, num_cells.1 * 3);

    let resized_image = picture.resize_exact(braille_size.0, braille_size.1, Lanczos3);
    let bw_image = resized_image.into_luma16();

    let mut buf: Vec<Vec<u8>> = vec![vec![0; num_cells.0 as usize]; num_cells.1 as usize];
    for (x, y, pixel) in bw_image.enumerate_pixels() {
        let pixel_value = pixel.0[0];
        let pixel_on = pixel_value > 32768;
        if pixel_on {
            let row = y / 3;
            let col = x / 2;
            let braille_row = y % 3;
            let braille_col = x % 2;
            buf[row as usize][col as usize] += 1 << (3 * braille_col + braille_row);
        }
    }

    buf
}

fn get_cells_size(width: u32, height: u32) -> (u32, u32) {
    let pic_size = (width as f32, height as f32);
    let term_size = size().unwrap();

    let bounded_term_size = ((term_size.0) as f32 / 2.0, (term_size.1 - 1) as f32);
    let scale_factors = (
        pic_size.0 / bounded_term_size.0,
        pic_size.1 / bounded_term_size.1,
    );
    let scale_factor = f32::max(scale_factors.0, scale_factors.1);

    (
        2 * (pic_size.0 / scale_factor).floor() as u32,
        (pic_size.1 / scale_factor).floor() as u32,
    )
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let my_image = ImageReader::open(&argv[1]).unwrap().decode().unwrap();
    for row in make_braille_pic(my_image) {
        for braille_index in row {
            print!("{}", BRAILLE[braille_index as usize]);
        }
        println!()
    }
}
