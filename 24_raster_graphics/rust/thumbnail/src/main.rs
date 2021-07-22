use image::{imageops::FilterType, ImageFormat};
use std::path::PathBuf;

fn julia() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        let cx = y as f32 * scalex - 1.5;
        let cy = x as f32 * scaley - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut g = 0;
        while g < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            g += 1;
        }
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("fractal.png").unwrap();
}

fn main() {
    julia();
    for fname in std::env::args().skip(1) {
        let src_path = PathBuf::from(&fname);
        let mut thumb_path = PathBuf::from(&fname);
        thumb_path.set_extension("thumbnail.jpg");
        if src_path != thumb_path {
            let src_img = image::open(&src_path).expect("Can't open input image.");
            let src_img = image::imageops::resize(&src_img, 128, 128, FilterType::CatmullRom);
            src_img
                .save_with_format(&thumb_path, ImageFormat::Jpeg)
                .unwrap();
        }
    }
}
