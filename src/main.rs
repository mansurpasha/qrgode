use std::io;

fn main() {
    let mut buf = String::new();

    println!("Enter string to convert:");

    io::stdin()
        .read_line(&mut buf)
        .expect("Error reading line");

    let _code = qrgode::create_qr_code(buf);

    create_image();
}

fn create_image() {
    let mut img = image::ImageBuffer::new(500, 500);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb::<u8>([20, 20, 20]);
    }
    img.save("test.png").unwrap();
}

