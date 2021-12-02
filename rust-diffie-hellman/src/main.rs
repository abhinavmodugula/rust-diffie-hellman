use image::RgbImage;
use image::ImageBuffer;
use rand::Rng;
use image::GenericImageView;
use image::Pixel;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;
use image::Rgb;

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
    let mut image: RgbImage = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);
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
    println!("{:?}", image.get_pixel(0,0).to_rgb().0);
    image.save(filename.to_owned()+".png").unwrap(); //save the encyrpted image for later
    //println!("{:?}", image::open(filename.to_owned()+".png").unwrap().get_pixel(0,0).to_rgb().0);
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

    image.save(filename.to_string()).unwrap(); //save the encyrpted image for later

}

fn encrypt_text(filename: &str, encryptfile: &str) {
  let rand_vec = reader(filename);
  let mut f = std::fs::File::open(encryptfile).expect("Unable to create file");
  let mut contents = String::new();
  f.read_to_string(&mut contents).unwrap();
  let contents = contents.as_bytes();
  println!("{:?}", contents);
  //let mut f = File::create(encryptfile).expect("Unable to create file");
  //for index in 0..contents.len() {
    //let char = contents[index] ^ rand_vec[index];
    //println!("{} = {} ^ {}", char, rand_vec[index], contents[index]);
    //write!(f, "{}", char as char);
  //}
}

fn main() { 
    pick_random("secret");
    encrypt("f1.jpeg", ".secret");
    decrypt("encrypted_f1.jpeg", ".secret");
    //encrypt_text(".secret", "hello.txt");
}


