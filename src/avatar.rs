use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use image::FilterType;
use image::ImageResult;
use image::DynamicImage;

pub fn convert_buf(buf: &[u8], out: &PathBuf, name: &str) -> Result<(), String> {
    let img = image::load_from_memory(buf).map_err(|e| format!("{}", e))?;
    convert(&img, out, name)
}

pub fn convert_path(path: &PathBuf, out: &PathBuf, name: &str) -> Result<(), String> {
    let img = open_magic(path).map_err(|e| format!("({}) {}", path.to_string_lossy(), e))?;
    convert(&img, out, name)
}

pub fn convert(img: &DynamicImage, out: &PathBuf, name: &str) -> Result<(), String> {
    for i in &[230, 100, 40] {
        let down_sized = img.resize_to_fill(*i, *i, FilterType::CatmullRom);
        let mut path = out.clone();
        path.push(format!("{}", i));
        fs::create_dir_all(&path).map_err(|e| format!("{}", e))?;
        path.push(String::from(name));
        down_sized
            .save(path)
            .map_err(|e| format!("error writing file ({}) for {}: {}", i, name, e))?;
    }
    Ok(())
}

fn open_magic(path: &PathBuf) -> ImageResult<DynamicImage> {
    let fin = match File::open(path) {
        Ok(f) => f,
        Err(err) => return Err(image::ImageError::IoError(err)),
    };
    let mut fin = BufReader::new(fin);

    let format = image::guess_format(fin.fill_buf().map_err(|e| image::ImageError::from(e))?)?;
    image::load(fin, format)
    
}
