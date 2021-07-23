#[macro_use]
extern crate clap;
use clap::App;
use std::fs;
use std::error::Error;
use image::io::Reader as ImageReader;

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
            let pos:Vec<&str> = filename.split(".").collect();
            let filetype = pos[pos.len()-1];
            if filetype.eq("png") {
                let img = ImageReader::open(filename)?.decode()?;
                println!("{:#?}", img);
            } else if filetype.eq("txt") {
                let content = fs::read_to_string(filename);
                println!("{:#?}", content);
            }
        }
    } else {
        println!("请输入指令 \n");
    }

    Ok(())
}