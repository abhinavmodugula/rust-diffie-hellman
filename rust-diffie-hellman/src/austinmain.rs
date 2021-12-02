use image::RgbaImage;
use image::ImageBuffer;
use rand::Rng;
use image::GenericImageView;

fn do_xor(g: &[u8], s: &[u8]) -> Vec<u8> {
	let mut vec = Vec::new();
	for i in 0..s.len() {
		vec.push(s[i]^g[i%g.len()]);	
	}
	vec
}

fn new_test() {
    let mut rng = rand::thread_rng();
    let mut rand_vec = Vec::new();
    for i in 0..rng.gen_range(20000..50000) {
	let new_base: u64 = rng.gen_range(0..2_u64.pow(63));
	let new_base_vec = new_base.to_ne_bytes();
	rand_vec.append(&mut new_base_vec.to_vec());
    }
    println!("{:?}", rand_vec);
    let img = image::open("f1.jpeg").unwrap();
    let (w, h) = img.dimensions();
    let mut out: RgbaImage = ImageBuffer::new(w, h);
    let mut count = 0;
    for pixel in img.pixels() {
        let mut other_pixel = out.get_pixel_mut(pixel.0, pixel.1);
        let orig_pixel = pixel.2.clone().0;
	let mut new_vals: [u8;4] = [0, 0, 0, 0];
	for i in 0..4 {
	    new_vals[i] = orig_pixel[i]^rand_vec[count%rand_vec.len()];
	}
        println!("{}, {}", orig_pixel[3], new_vals[3]);
     	count += 1;
    	*other_pixel = image::Rgba(new_vals);
    }

    println!("dimensions of output image: {:?}", out.dimensions());
    out.save("encrypted_f1.jpeg").unwrap(); //save the encyrpted image for later


    //DECRYPT TIME!!!!
    let img = image::open("encrypted_f1.jpeg").unwrap();
    let (w, h) = img.dimensions();
    let mut out: RgbaImage = ImageBuffer::new(w, h);
    let mut count = 0;
    for pixel in img.pixels() {
        let mut other_pixel = out.get_pixel_mut(pixel.0, pixel.1);
        let orig_pixel = pixel.2.clone().0;
        let mut new_vals: [u8;4] = [0, 0, 0, 0];
        for i in 0..3 { 
            new_vals[i] = orig_pixel[i]^rand_vec[count%rand_vec.len()];
        }
	new_vals[3] = 255;
        println!("{}, {}", orig_pixel[3], new_vals[3]);
        count += 1;
        *other_pixel = image::Rgba(new_vals);
    }
    
    println!("dimensions of output image: {:?}", out.dimensions());
    out.save("decrypted_f1.jpeg").unwrap();
}

fn main() {
    new_test();
}
