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

mod general;
mod diffie_hellman;


fn main() { 

    let args: Vec<String> = env::args().collect();

    if &args[1] == "encrypt" {
        diffie_hellman::encrypt(&args[2], &args[3]);
    }

    if &args[1] == "decrypt" {
        diffie_hellman::decrypt(&args[2], &args[3]);
    }

    if &args[1] == "encrypt_text" {
        diffie_hellman::encrypt_text(&args[3], &args[2]);
    }

    if &args[1] == "decrypt_text" {
        diffie_hellman::decrypt_text(&args[3], &args[2]);
    }
    
    if &args[1] == "pick_random" {
        general::pick_random(&args[2]);
    }

}


