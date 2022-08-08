use std::io;

pub mod utils;
pub mod render;

fn main() {
    let mut string_to_convert = String::new();

    println!("Enter string to convert:");
    io::stdin()
        .read_line(&mut string_to_convert)
        .expect("Error reading string to convert");

    let code = utils::create_qr_code(string_to_convert);

    render::convert_qr_code_to_image(code, "output.png");
}


