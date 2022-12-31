mod args;

use args::Args;
use std::fs::File;
use std::io::BufReader;
use image::{io::Reader, DynamicImage, ImageFormat, GenericImageView, imageops::Triangle};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
}

fn main() -> Result <(), ImageDataErrors>{
    /*
        // This method of declaring a new Args struct will turn out to be quite uncomfortable. So we should abstract it a function implemented for this struct specifically.
        let args: Args = Args {
            image_1: String::new(), // This `new` function has been implemented specifically for this `String` struct.
            image_2: String::new(),
            output: String::new()
        };
    */
    let args: Args = Args::new();
    /*
        // println!("{}", args); // This throws an error, since println macro does not know how to format the Args struct type.
        // println!("{:#?}", args); // Since the above did not work, let's try extending the formatter.
        // This also did not work since the trait `Debug` was not implemented for `args::Args`. So we will try printing again after implementing the trait Debug for Args
    */
    println!("{:#?}", args);
    println!("Hello, world!");

    // Let's find the image_path and image_format from the Command Line arguments
    let (image_1, image_1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);

    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    } 

    // This will resize the larger image to the dimensions of the smaller image
    let(image_1, image_2) = standardise_image_size(image_1, image_2);

    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smaller_dimensions(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pixels_1 = dim1.0 * dim1.1;
    let pixels_2 = dim2.0 * dim2.1;

    return if pixels_1 > pixels_2 { dim1 } else { dim2 };
}

fn standardise_image_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smaller_dimensions(image_1.dimensions(), image_2.dimensions());

    if image_1.dimensions() == (width, height) {
        (image_1, image_2.resize_exact(width, height, Triangle))
    } else {
        (image_1.resize_exact(width, height, Triangle), image_2)
    }
}