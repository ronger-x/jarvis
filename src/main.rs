#[macro_use]
extern crate clap;
extern crate image;
extern crate colorful;

use clap::App;
use std::fs;
use std::error::Error;
use image::io::Reader as ImageReader;

use image::{DynamicImage, GenericImageView};
use colorful::Colorful;
use colorful::RGB;
use colorful::core::color_string::CString;

fn main() -> Result<(), Box<dyn Error>> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("jarvis.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Same as previous examples...
    if let Some(params) = matches.values_of("read") {
        let p_list: Vec<&str> = params.collect();
        let filename = p_list[0]; // 参数一
        if filename.is_empty() {
            println!("请输入文件名 \n");
        } else {
            let pos: Vec<&str> = filename.split(".").collect();
            let filetype = pos[pos.len() - 1];
            if filetype.eq("png") {
                return read_png(filename);
            } else if filetype.eq("txt") {
                let content = fs::read_to_string(filename);
                println!("{:#?}", content?);
            }
        }
    } else {
        println!("请输入指令 \n");
    }

    Ok(())
}

fn read_png(path: &str) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(path)?.decode()?;
    if img.height() > 50 {
        img = img.thumbnail(50, 50);
    }
    match img {
        DynamicImage::ImageLuma8(image) => {
            println!("ImageLuma8: width: {}, height: {}", image.width(), image.height());
        }
        DynamicImage::ImageLumaA8(image) => {
            println!("ImageLumaA8: width: {}, height: {}", image.width(), image.height());
        }
        DynamicImage::ImageRgb8(image) => {
            println!("ImageRgb8: width: {}, height: {}", image.width(), image.height());
            let mut y = 0;
            while y < image.height() {
                if y > 0 {
                    println!("\x1b[0m");
                }
                for x in 0..image.width() {
                    let p1 = image.get_pixel(x, y);
                    let p2 = image.get_pixel(x, y + 1);
                    print!("{}", get_pixel_str(RGB::new(p1[0], p1[1], p1[2]), RGB::new(p2[0], p2[1], p2[2])));
                }
                y += 2;
            }
        }
        DynamicImage::ImageRgba8(image) => {
            println!("ImageRgba8: width: {}, height: {}", image.width(), image.height());
            let mut y = 0;
            while y < image.height() {
                if y > 0 {
                    println!("\x1b[0m");
                }
                for x in 0..image.width() {
                    let p1 = image.get_pixel(x, y);
                    let p2 = image.get_pixel(x, y + 1);
                    if p1[3] == 0 {
                        print!("\x1b[0m ");
                    } else {
                        print!("{}", get_pixel_str(RGB::new(p1[0], p1[1], p1[2]), RGB::new(p2[0], p2[1], p2[2])));
                    }
                }
                y += 2;
            }
        }
        DynamicImage::ImageBgr8(image) => {
            println!("ImageBgr8: width: {}, height: {}", image.width(), image.height());
        }
        DynamicImage::ImageBgra8(image) => {
            println!("ImageBgra8: width: {}, height: {}", image.width(), image.height());
        }
        DynamicImage::ImageLuma16(image) => {
            println!("ImageLuma16: width: {}, height: {}", image.width(), image.height());
        }
        DynamicImage::ImageLumaA16(image) => {
            println!("ImageLumaA16: width: {}, height: {}", image.width(), image.height());
        }
        DynamicImage::ImageRgb16(image) => {
            println!("ImageRgb16: width: {}, height: {}", image.width(), image.height());
        }
        DynamicImage::ImageRgba16(image) => {
            println!("ImageRgba16: width: {}, height: {}", image.width(), image.height());
        }
    }
    Ok(())
}

fn get_pixel_str(rgb1: RGB, rgb2: RGB) -> CString {
    const CHAR_HALF_BLOCK: &str = "▄";
    String::from(CHAR_HALF_BLOCK)
        .color(rgb2)
        .bg_color(rgb1)
}