extern crate x11_screenshot;

fn main() {
    let mut args = std::env::args().skip(1);
    let x = args.next().expect("Need x position").parse().unwrap();
    let y = args.next().expect("Need y position").parse().unwrap();
    let screen = x11_screenshot::Screen::open().expect("Failed to open screen");
    let x11image = screen.capture().expect("Failed to take screenshot");
    let bgr = x11image.pixel_at(x, y);
    println!("r: {} g: {} b: {}", bgr.r, bgr.g, bgr.b);
}
