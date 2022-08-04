use std::io;
use graphics::{ellipse, types::Color};
use graphics_buffer::{IDENTITY, RenderBuffer};
use qrcode::QrCode;

fn main() {
    let mut buf = String::new();

    println!("Enter string to convert:");

    io::stdin()
        .read_line(&mut buf)
        .expect("Error reading line");

    let code = qrgode::create_qr_code(buf);

    convert_qr_code_to_image(code,"test.png");
}

fn convert_qr_code_to_image(qr_code: QrCode, filename: &str) {
    let code_width = qr_code.width();
    let qr_code_matrix = qr_code.into_vec();
    
    let circle_radius: usize = 10; 
    let image_width = circle_radius * 2 * code_width;
    let mut buffer = RenderBuffer::new(image_width as u32, image_width as u32);

    // Set background
    buffer.clear(BROWN);

    // Draw circles
    for x in 0..code_width {
        for y in 0..code_width {
            let item = code_width * y + x;
            let diameter = circle_radius * 2;
            let centre_x = diameter * x + circle_radius;
            let centre_y = diameter * y + circle_radius;
            println!("Code width is {}, x is {}, y is {}, item is {}", code_width, x, y, item);
            let color = if qr_code_matrix[item] {
                BLACK
            } else {
                WHITE
            };
            let circle = Circle {
                centre: Point { x: centre_x as f64, y: centre_y as f64 },
                radius: circle_radius as f64,
                color,
            };
            draw_circle(&mut buffer, circle);
        }
    }

    buffer.save(filename).unwrap();
}

const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: Color = [0.9, 0.9, 0.9, 1.0];
const BROWN: Color = [139.0/255.0, 69.0/255.0, 19.0/255.0, 1.0];

struct Point {
    x: f64,
    y: f64,
}

struct Circle {
    centre: Point,
    radius: f64,
    color: Color,
}

fn draw_circle(buffer: &mut RenderBuffer, circle: Circle) {
    let Circle { centre, radius, color } = circle;
    ellipse(
        color,
        [centre.x - circle.radius, centre.y - radius, radius * 2.0, radius * 2.0],
        IDENTITY,
        buffer,
    );
}

