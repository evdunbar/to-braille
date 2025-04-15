use crossterm::terminal::size;
const BRAILLE: [[&str; 16]; 4] = [
    [
        // U+280x
        "⠀", "⠁", "⠂", "⠃", "⠄", "⠅", "⠆", "⠇", "⠈", "⠉", "⠊", "⠋", "⠌", "⠍", "⠎", "⠏",
    ],
    [
        // U+281x
        "⠐", "⠑", "⠒", "⠓", "⠔", "⠕", "⠖", "⠗", "⠘", "⠙", "⠚", "⠛", "⠜", "⠝", "⠞", "⠟",
    ],
    [
        // U+282x
        "⠠", "⠡", "⠢", "⠣", "⠤", "⠥", "⠦", "⠧", "⠨", "⠩", "⠪", "⠫", "⠬", "⠭", "⠮", "⠯",
    ],
    [
        // U+283x
        "⠰", "⠱", "⠲", "⠳", "⠴", "⠵", "⠶", "⠷", "⠸", "⠹", "⠺", "⠻", "⠼", "⠽", "⠾", "⠿",
    ],
];

// picture needs to end up as multiple of 2 x multiple of 3
// also should fit in current terminal size
fn calculate_size() {
    let pic_size = (1920, 1280);
    let term_size = size().unwrap();

    let pic_landscape = pic_size.0 >= pic_size.1;
    if pic_landscape {
        let scale_factor = pic_size.0 / term_size.0;
    } else {
        let scale_factor = pic_size.1 / term_size.1;
    }
}

fn main() {
    println!("{:?}", size());
}
