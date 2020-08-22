extern crate image;
extern crate raster;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod image_kernel;

#[wasm_bindgen]
pub fn anime4k(buf : &[u8]) -> Vec<u8> {
  let iteration = 1;
  let scale = 2.0;
  let push_color_strength = 0.0;
  let push_gradient_strength = 1.0;

  println!("Opening image");
  let image = image::load_from_memory(buf).expect("Can't open image.");

  println!("Creating instance");
  let mut kernel_instance = image_kernel::ImageKernel::from_image(image);
  
  println!("Scaling image");
  kernel_instance.scale(
      (kernel_instance.width() as f64 * scale) as u32,
      (kernel_instance.height() as f64 * scale) as u32,
  );

  for _ in 0..iteration {
      println!("Compute luminance");
      kernel_instance.compute_luminance();
      println!("Push color");
      kernel_instance.push_color(image_kernel::clamp(
          (push_color_strength * 255.0) as u16,
          0,
          0xFFFF,
      ));
      println!("Compute gradient");
      kernel_instance.compute_gradient();
      println!("Push gradient");
      kernel_instance.push_gradient(image_kernel::clamp(
          (push_gradient_strength * 255.0) as u16,
          0,
          0xFFFF,
      ));
  }
  println!("Finalizing");

    let img_width = kernel_instance.image.width();
    let img_height = kernel_instance.image.height();
    let bytes = kernel_instance
      .save();
    let mut out = Vec::<u8>::new();
    image::png::PNGEncoder::new(&mut out)
        .encode(&bytes[..], img_width, img_height, image::ColorType::Rgba8)
        .unwrap();
    out
}
