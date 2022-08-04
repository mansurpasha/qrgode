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
    buffer.clear(BROWN);

    let black_circle = Circle { centre: Point { x: 50.0, y: 50.0 }, radius: 50.0, color: BLACK };
    draw_circle(&mut buffer, black_circle);
    let white_circle = Circle { centre: Point { x: 25.0, y: 25.0 }, radius: 25.0, color: WHITE };
    draw_circle(&mut buffer, white_circle);

    buffer.save("test.png").unwrap();
}

struct Point {
    x: f64,
    y: f64,
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}

const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const BROWN: Color = [222.0/255.0, 184.0/255.0, 135.0/255.0, 1.0];

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

