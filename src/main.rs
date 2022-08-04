use std::io;
use graphics::{ellipse, types::Color};
use graphics_buffer::{IDENTITY, RenderBuffer};

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
    let mut buffer = RenderBuffer::new(100, 100);
    // Brown background
    buffer.clear([222.0, 184.0, 135.0, 1.0]);

    let centre = Point::new(50.0, 50.0);
    draw_circle(&mut buffer, centre, 50.0, [1.0, 0.0, 0.0, 1.0]);

    buffer.save("test.png").unwrap();
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y:f64) -> Point {
        Point { x, y }
    }
}

fn draw_circle(buffer: &mut RenderBuffer, centre: Point, radius: f64, color: Color) {
    ellipse(
        color,
        [centre.x - radius, centre.y - radius, radius * 2.0, radius * 2.0],
        IDENTITY,
        buffer,
    );
}

