use image::RgbImage;
use image::ImageBuffer;
use rand::Rng;
use image::GenericImageView;
use image::Pixel;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use image::Rgb;
use std::env;

use crate::general::reader;

pub fn encrypt(filename: &str, encryptfile: &str) {
    let rand_vec = reader(encryptfile);
    let img = image::open(filename.to_string()).unwrap();
    let (w, h) = img.dimensions();
    let mut image: RgbImage = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);
    let mut count = 0;
    for x in 0..w {
    for y in 0..h {
        let mut new_vals: [u8;3] = [0, 0, 0];
        for i in 0..3 {
                new_vals[i] = img.get_pixel(x, y).to_rgb().0[i]^rand_vec[(3*count+i)%rand_vec.len()];
        }
            image[(x,y)] = image::Rgb(new_vals);
        count += 1;
        }
    }
    image.save(filename.to_owned()).unwrap(); //save the encyrpted image for later
    //fs::remove_file(filename);
}

pub fn decrypt(filename: &str, encryptfile: &str) {

    let rand_vec = reader(encryptfile);
    let img = image::open(filename.to_string()).unwrap();
    let (w, h) = img.dimensions();
    let mut image: RgbImage = ImageBuffer::new(w, h);
    let mut count = 0;
    for x in 0..w {
        for y in 0..h {
            let mut new_vals: [u8;3] = [0, 0, 0];
            for i in 0..3 {
                new_vals[i] = img.get_pixel(x, y).to_rgb().0[i]^rand_vec[(3*count+i)%rand_vec.len()];
            }
            image[(x,y)] = image::Rgb(new_vals);
            count += 1;
        }
    }

    image.save(filename.to_string()).unwrap(); //save the encyrpted image for later

}

pub fn encrypt_text(filename: &str, encryptfile: &str) {
    let mut rand_vec = reader(filename);
    rand_vec = rand_vec.into_iter().map(|x| x % 128).collect();
    let mut f = File::open(encryptfile).expect("Unable to create file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let contents = contents.as_bytes();
    let mut f = File::create(encryptfile).expect("Unable to create file");
    for index in 0..contents.len() {
    let char = contents[index] ^ rand_vec[index];
    write!(f, "{}", char as char);
    }
}

pub fn decrypt_text(filename: &str, encryptfile: &str) {
    let mut rand_vec = reader(filename);
    rand_vec = rand_vec.into_iter().map(|x| x % 128).collect();
    let mut f = File::open(encryptfile).expect("Unable to create file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let contents = contents.as_bytes();
    let mut f = File::create(encryptfile).expect("Unable to create file");
    for index in 0..contents.len() {
    let char = contents[index] ^ rand_vec[index];
    write!(f, "{}", char as char);
    }
}