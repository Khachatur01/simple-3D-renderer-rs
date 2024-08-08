use crate::rendering_engine::engine::pixel::Pixel;

/**
 * converting colors from range 0..255 to range 0..1, making a color blending, then converting back to range 0..255
 */
pub fn blend_color(background: Pixel, foreground: Pixel) -> Pixel {
    let foreground_r: f32 = foreground.color.r as f32 / 255.0;
    let foreground_g: f32 = foreground.color.g as f32 / 255.0;
    let foreground_b: f32 = foreground.color.b as f32 / 255.0;
    let foreground_a: f32 = foreground.color.a as f32 / 255.0;

    let background_r: f32 = background.color.r as f32 / 255.0;
    let background_g: f32 = background.color.g as f32 / 255.0;
    let background_b: f32 = background.color.b as f32 / 255.0;
    let background_a: f32 = background.color.a as f32 / 255.0;

    let result_r: f32 = (foreground_r * foreground_a) + (background_r * (1.0 - background_a));
    let result_g: f32 = (foreground_g * foreground_a) + (background_g * (1.0 - background_a));
    let result_b: f32 = (foreground_b * foreground_a) + (background_b * (1.0 - background_a));

    Pixel::new(
        (result_r * 255.0) as u8,
        (result_g * 255.0) as u8,
        (result_b * 255.0) as u8,
        255
    )
}