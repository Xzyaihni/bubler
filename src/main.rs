use std::{
    env,
    process,
    path::Path
};

use image::{
    DynamicImage,
    RgbaImage,
    imageops::FilterType
};


fn complain(message: &str) -> !
{
    eprintln!("{message}");

    process::exit(1)
}

fn open_image<P: AsRef<Path>>(path: P) -> DynamicImage
{
    image::open(path).unwrap_or_else(|err|
    {
        complain(&format!("error opening image: {err:?}"))
    })
}

#[allow(dead_code)]
fn weird_star(x: f32, y: f32, size: f32) -> bool
{
    let value = y.abs().powi(2).cbrt() + x.abs().powi(2).cbrt();

    value < (size.sqrt() * 1.8)
}

fn circle(x: f32, y: f32, size: f32) -> bool
{
    y.hypot(x) < (size / 2.0)
}

fn shape_test(x: f32, y: f32, size: f32) -> bool
{
    circle(x, y, size)
}

fn bubly_mix(
    main_image: DynamicImage,
    second_image: DynamicImage,
    circle_size: f32
) -> RgbaImage
{
    let mut main_image = main_image.into_rgba8();
    let (width, height) = (main_image.width(), main_image.height());

    let second_image = second_image
        .resize_exact(width, height, FilterType::Lanczos3)
        .into_rgba8();

    let circle_size = circle_size * width.min(height) as f32;

    let offset_ratio = 1.5;
    let offset = 2.5;

    let total_cells_y = (height as f32 / circle_size) - (offset * offset_ratio);

    for y in 0..height
    {
        for x in 0..width
        {
            let main_pixel = main_image.get_pixel_mut(x, y);
            let secondary_pixel = second_image.get_pixel(x, y);

            let cell_y = (y as f32 / circle_size).floor() - offset;

            let height_fraction = 1.0 - (cell_y / total_cells_y);

            let this_circle_size = circle_size * height_fraction;

            let in_circle = |coordinate|
            {
                (coordinate as f32 % circle_size) - (circle_size / 2.0)
            };

            let circle_x = in_circle(x);
            let circle_y = in_circle(y);

            let secondary = shape_test(circle_x, circle_y, this_circle_size);

            if secondary
            {
                *main_pixel = *secondary_pixel;
            }
        }
    }

    main_image
}

fn main()
{
    let mut args = env::args().skip(1);

    let first_image_path = args.next()
        .unwrap_or_else(|| complain("plz provide first filepath as argument"));

    let second_image_path = args.next()
        .unwrap_or_else(|| complain("plz provide second filepath as argument"));

    let circle_size = args.next().map(|circle_size| circle_size.parse().ok())
        .flatten().unwrap_or(0.1);

    let portrait = true;
    let filtered_open = |path|
    {
        let image = open_image(path);

        if portrait
        {
            image.rotate270()
        } else
        {
            image
        }
    };

    let (first_image, second_image) =
        (filtered_open(first_image_path), filtered_open(second_image_path));

    let output_image = bubly_mix(first_image, second_image, circle_size);

    output_image.save("output.png").unwrap_or_else(|err|
    {
        complain(&format!("error saving image: {err:?}"))
    })
}