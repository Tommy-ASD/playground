use image::{save_buffer, GrayImage, ImageBuffer, Luma};
use imageproc::drawing::{draw_text_mut, text_size};

use rusttype::{Font, Scale};

fn main() {
    let font = std::fs::read("sans.ttf").unwrap();
    let font = Font::try_from_vec(font).unwrap();

    let (image, w, h) = text_image("Hello, World!", font, 100.0, 100.0);

    save_gray_image(image, w, h);

    dbg!()
}

fn text_image(
    text: &str,
    font: Font,
    font_width: f32,
    font_height: f32,
) -> (ImageBuffer<Luma<u8>, Vec<u8>>, u32, u32) {
    let scale = Scale {
        x: font_width,
        y: font_height,
    };

    let (mut w, h) = text_size(scale, &font, text);

    if w % 8 != 0 {
        w += 8 - (w % 8);
    }

    println!("text_image: result size {}x{}", w, h);

    let mut image: image::ImageBuffer<Luma<u8>, Vec<u8>> = GrayImage::new(w as _, h as _);

    draw_text_mut(&mut image, Luma([255u8]), 0, 0, scale, &font, text);

    (image, w.try_into().unwrap(), h.try_into().unwrap())
}

fn save_gray_image(image: image::ImageBuffer<image::Luma<u8>, Vec<u8>>, width: u32, height: u32) {
    let raw = image.into_raw();

    dbg!(&width, &height, &raw.len(), height * width);

    // Save the image to a file (in this example, as a PNG)
    if let Err(e) = save_buffer("output.png", &raw, width, height, image::ColorType::L8) {
        eprintln!("Error saving image: {}", e);
    }
}
