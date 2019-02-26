extern crate image;
extern crate x11_screenshot;

fn main() {
    let screen = x11_screenshot::Screen::open().expect("Failed to open screen");
    let x11image = screen.capture().expect("Failed to take screenshot");
    let rgb_image: image::RgbImage = x11image.into();
    // Save image
    // For documentation on the image crate, see http://www.piston.rs/image/image/index.html
    rgb_image.save("example_screenshot.png").unwrap();
}
