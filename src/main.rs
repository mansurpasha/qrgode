extern crate cairo;

use std::{io, fs::File, f64::consts::PI};
use qrcode::QrCode;
use cairo::{ImageSurface, Format, Context};

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
    
    let circle_radius: usize = 8; 
    let image_width = circle_radius * 2 * code_width;
    let border: i32 = 5;

    let surface = ImageSurface::create(Format::ARgb32, image_width as i32 + border * 2, image_width as i32 + border * 2).unwrap();
    let context = Context::new(&surface).unwrap();

    // Set background colour
    context.set_source_rgb(BROWN.0, BROWN.1, BROWN.2);
    context.paint().expect("Error painting to surface");

    // Draw circles
    for y in 0..code_width {
        for x in 0..code_width {
            let item = code_width * y + x;
            let diameter = circle_radius * 2;
            let centre_x = diameter * x + circle_radius + border as usize;
            let centre_y = diameter * y + circle_radius + border as usize;
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
            draw_circle(&context, circle);
        }
    }

    let mut file = File::create(filename).unwrap();
    surface.write_to_png(&mut file).unwrap();
}

const BLACK: (f64, f64, f64) = (0.0, 0.0, 0.0);
const WHITE: (f64, f64, f64) = (0.9, 0.9, 0.9);
const BROWN: (f64, f64, f64) = (139.0/255.0, 69.0/255.0, 19.0/255.0);

struct Point {
    x: f64,
    y: f64,
}

struct Circle {
    centre: Point,
    radius: f64,
    color: (f64, f64, f64),
}

fn draw_circle(context: &Context, circle: Circle) {
    let Circle { centre, radius, color } = circle;
    context.set_line_width(0.02);
    context.set_source_rgb(color.0, color.1, color.2);
    context.arc(centre.x, centre.y, radius-0.8, 0.0, PI*2.);
    context.fill_preserve().unwrap();
    context.stroke().unwrap();
}

