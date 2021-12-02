use image::RgbImage;
use image::ImageBuffer;
use rand::Rng;
use image::GenericImageView;
use image::Pixel;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;

fn pick_random (encryptfile: &str) {
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

fn reader(encryptfile: &str) -> Vec<u8> {
  let mut file = std::fs::File::open(encryptfile.to_string()).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut v: Vec<u8> = Vec::new();
  for s in contents.split(",") {
    v.push(s.trim().parse::<u8>().unwrap());
  }
  v
}

fn encrypt(filename: &str, encryptfile: &str) {

    let rand_vec = reader(encryptfile);
    let img = image::open(filename.to_string()).unwrap();
    let (w, h) = img.dimensions();
    let mut image: RgbImage = ImageBuffer::new(w, h);
    let mut count = 0;
    for x in 0..w {
	for y in 0..h {
	    let mut new_vals: [u8;3] = [0, 0, 0];
	    for i in 0..3 {
	     	new_vals[i] = img.get_pixel(x, y).to_rgb().0[i]^rand_vec[count%rand_vec.len()];
	    }
    	    image[(x,y)] = image::Rgb(new_vals);
	    count += 1;
    	}
    }

    image.save("encrypted_".to_owned()+filename).unwrap(); //save the encyrpted image for later
}

fn decrypt(filename: &str, encryptfile: &str) {

    let rand_vec = reader(encryptfile);
    let img = image::open(filename.to_string()).unwrap();
    let (w, h) = img.dimensions();
    let mut image: RgbImage = ImageBuffer::new(w, h);
    let mut count = 0;
    for x in 0..w {
        for y in 0..h {
            let mut new_vals: [u8;3] = [0, 0, 0];
            for i in 0..3 {
                new_vals[i] = img.get_pixel(x, y).to_rgb().0[i]^rand_vec[count%rand_vec.len()];
            }
            image[(x,y)] = image::Rgb(new_vals);
            count += 1;
        }
    }

    image.save("decrypted_".to_owned()+filename).unwrap(); //save the encyrpted image for later

}

fn main() { 
    pick_random("secret");
    encrypt("f1.jpeg", ".secret");
    decrypt("encrypted_f1.jpeg", ".secret");
}
