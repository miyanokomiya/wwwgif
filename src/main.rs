use std::fs::File;
use std::io::prelude::*;

// GIF89a
const GIF_SIGNATURE: [u8; 6] = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61];

fn main() -> std::io::Result<()> {
    {
        let mut file = File::create("./tmp/test")?;
        // Write a slice of bytes to the file
        // file.write_all(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])?;

        file.write_all(&GIF_SIGNATURE)?;

        let logical_screen_descriptor_width = 0x10;
        let logical_screen_descriptor_height = 0x10;
        let global_color_table_flag = 0b11110111;
        let background_color_index = 0x00;
        let pixel_aspect_ratio = 0x00;
        file.write_all(&[
            logical_screen_descriptor_width,
            0x00,
            logical_screen_descriptor_height,
            0x00,
            global_color_table_flag,
            background_color_index,
            pixel_aspect_ratio,
        ])?;
    }

    {
        let mut file = File::open("./tmp/test")?;
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        println!("{:?}", buffer);
    }

    Ok(())
}
