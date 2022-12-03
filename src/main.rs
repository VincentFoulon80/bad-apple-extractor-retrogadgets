use std::{fs::File, io::Write};

use image::{imageops::FilterType, GenericImageView};

fn main() {
    // please be aware that in production code all the unwrap() functions must be avoided
    // and replaced with proper error handling. I'm just doing prototype code here.

    // create a "data.lua" file that'll receive the output
    let mut file = File::create("data.lua").unwrap();
    // add lua variable declaration for the data stream
    file.write_all("stream = {}\nstream.data = {\n".as_bytes())
        .unwrap();

    // initialize variables used for compressing data
    let mut bit = false;
    let mut bitcount = 0;
    let mut data = vec![];

    // loop through all the frames
    for i in 1..=6572 {
        // open the image "output_{frame}.jpg"
        let img = image::open(&format!("frames/output_{:04}.jpg", i)).unwrap();
        // resize the image to fit the target screen
        let img = img.resize_exact(64, 64, FilterType::Lanczos3);

        // print the frame number to know where we're at
        println!("{}", i);

        // loop through each pixels (reading left to right, top to bottom)
        for (_, _, color) in img.pixels() {
            // the source is a monochromatic video, so we only care about the red channel here
            // the target is only two colors, so we check if the brightness is over half the maximum value (255)
            let current_bit = color.0[0] > 127;

            // the video is naively compressed by counting how many pixels are the same color. If the color changes,
            // save the current count to the data list and start over until the color change again
            if current_bit != bit {
                bit = current_bit;
                data.push(bitcount);
                bitcount = 0;
            }
            bitcount += 1;
        }
    }
    // we've iterated through the entire video, we just need to push one last time the count
    data.push(bitcount);

    // and then write all our collected values to the lua file, separated with commas
    file.write_all(
        data.iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join(", ")
            .as_bytes(),
    )
    .unwrap();

    // finish the lua file so you can import it with `require`
    file.write_all("\n}\nreturn stream\n".as_bytes()).unwrap();
}
