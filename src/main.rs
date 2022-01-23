use std::fs::File;
use std::io::prelude::*;

// GIF89a
const GIF_SIGNATURE: [u8; 6] = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61];

fn main() -> std::io::Result<()> {
    {
        let mut file = File::create("./tmp/test")?;

        // header
        {
            file.write_all(&GIF_SIGNATURE)?;
        }

        // descriptor
        {
            let logical_screen_descriptor_width = 10;
            let logical_screen_descriptor_height = 10;
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

        // global color table
        {
            let mut global_color_table = [Color::new_rgb(0, 0, 0); 256];
            global_color_table[0] = Color::new_rgb(255, 0, 0);
            global_color_table[1] = Color::new_rgb(0, 255, 0);
            global_color_table[2] = Color::new_rgb(0, 0, 255);
            global_color_table[3] = Color::new_rgb(255, 255, 255);

            let mut global_color_table_bytes = [0; 256 * 3];
            global_color_table
                .iter()
                .enumerate()
                .for_each(|(i, color)| {
                    let index = i * 3;
                    global_color_table_bytes[index] = color.red;
                    global_color_table_bytes[index + 1] = color.green;
                    global_color_table_bytes[index + 2] = color.blue;
                });
            file.write_all(&global_color_table_bytes)?;
        }
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

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}
