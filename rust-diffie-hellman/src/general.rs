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

pub fn pick_random (encryptfile: &str) {
    let mut rng = rand::thread_rng();
    let mut rand_vec = Vec::new();
    for i in 0..rng.gen_range(20000..50000) {
        let new_base: u64 = rng.gen_range(0..2_u64.pow(63));
        let new_base_vec = new_base.to_ne_bytes();
        rand_vec.append(&mut new_base_vec.to_vec());
    }
    let mut f = File::create(".".to_owned()+encryptfile).expect("Unable to create file");
    for i in 0..rand_vec.len()-1 {
        write!(f, "{},", rand_vec[i]);
    }
    write!(f, "{}", rand_vec[rand_vec.len()-1]);

}

pub fn reader(encryptfile: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(encryptfile.to_string()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut v: Vec<u8> = Vec::new();
    for s in contents.split(",") {
        v.push(s.trim().parse::<u8>().unwrap());
    }
    v
    }
