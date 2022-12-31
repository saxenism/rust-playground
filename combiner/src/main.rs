mod args;

use args::Args;
use std::fs::File;
use std::io::BufReader;
use std::convert::TryInto;
use image::{io::Reader, DynamicImage, ImageFormat, GenericImageView, imageops::Triangle};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = width * height * 4; // 4 is used becase of RGBA format of images. buffer_capacity is basically counting the number of pixels
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
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
    let mut output = FloatingImage::new(image_1.width(), image_2.height(), args.output);

    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?; // The ? syntax at the end of an expression is a shorthand way of handling the result of a function call. If the function call returns an error, the error propagation operator will return the error from the function call.

    // Saving the combined image
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgb8,
        image_1_format
    ).unwrap();

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

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()]; // This creates a vector of type u8 with vec1.len() number of 0's
                                                // On why we used vec_1 and not vec_2, well vec_1 and vec_2 both have the same length. So it doesn't matter which one you use
    //  Declared as mutable since this will start as a vector of all 0's but then will get values from vec_1 and vec_2.

    let mut i = 0; // iterator for the while loop. Will obv change values, hence defined as mutable

    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..i+4, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..i+4, set_rgba(&vec_2, i, i + 3));
        } // i..i+4 means a range starting from i and going to i + 3 (included) 
        i += 4;
    }

    combined_data // Return the combined_data vector containing combined data from vec_1 and vec_2
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    // The ..= syntax is Rust's range syntax which allows the range to be inclusive of the end value.
    for i in start..=end {
        let val = match vec.get(i) {
            Some(pixel_value) => *pixel_value, //The * symbol before a variable is Rust's dereferencing operator, which allows the value of the variable to be accessed. Since `get` was giving the pointer to the value at `i` position
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }

    rgba // Return rgba
}