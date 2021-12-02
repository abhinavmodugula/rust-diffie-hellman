fn test_image() {
    let mut rng = rand::thread_rng();
    
    //Initial set up
    let g: i64 = rng.gen_range(2000000000..5000500000000);
    let x: i64 = rng.gen_range(50000000..10000500000);
    let img = image::open("f1.jpeg").unwrap();
    println!("dimensions of original image: {:?}", img.dimensions());

    //Bob is sending the image
    let y: i64 = rng.gen_range(3..9);
    let com_sec: i64 = rng.gen_range(42..423);
    let inter = g^(com_sec*y) % x;

    //Now, apply the encryption to the image
    let (w, h) = img.dimensions();
    let mut out: RgbaImage = ImageBuffer::new(w, h);

    for pixel in img.pixels() {
        let mut other_pixel = out.get_pixel_mut(pixel.0, pixel.1);
        let orig_pixel = pixel.2.clone().0;
        
        let mut index = 0;
        let mut new_vals: [u8;4] = [0, 0, 0, 0];
        while index < 4 {
            let org_value = orig_pixel[index] as i64;
            new_vals[index] = (org_value^inter) as u8;
            index += 1;
        }

        *other_pixel = image::Rgba(new_vals);
    }

    println!("dimensions of output image: {:?}", out.dimensions());
    out.save("encrypted_f1.jpeg").unwrap(); //save the encyrpted image for later

    //now, decrypt the image (now, on Alice)
    let mut decrypted: RgbaImage = ImageBuffer::new(w, h);
    for xi in 0..w {
        for yi in 0..h {
            let pixel = out.get_pixel_mut(xi as u32, yi as u32);
            let mut other_pixel = decrypted.get_pixel_mut(xi as u32, yi as u32);
            let orig_pixel = pixel.0.clone();

            let ref_pixel = img.get_pixel(xi, yi).0;

            let mut index = 0;
            let mut new_vals: [u8;4] = [0, 0, 0, 0];
            while index < 4 {
                let org_value = orig_pixel[index] as i64;
                new_vals[index] = (org_value^inter) as u8;
                index += 1;
            }

            //println!("Should be equal: {:?} and {:?} ", new_vals, ref_pixel);
            
            *other_pixel = image::Rgba(new_vals);
        }
    }
    decrypted.save("decrypted_f1.jpeg").unwrap();
}

fn test_equality() {
    let mut rng= rand::thread_rng();

    //first, generate random mod and base
    let rand_mod: i64 = rng.gen_range(2000000000..5000500000000);
    let rand_base: i64 = rng.gen_range(50000000..10000500000);
    println!("Mod: {} Base: {}", rand_mod, rand_base);

    let alice_secret: i64 = rng.gen_range(3..9);
    let send_to_bob = rand_base^alice_secret % rand_mod;

    let bob_secret: i64 = rng.gen_range(3..9);
    let send_to_alice = rand_base^bob_secret % rand_mod;

    let alice_compute = send_to_alice^alice_secret % rand_mod;
    let bob_compute: i64 = send_to_bob^bob_secret % rand_mod;

    println!("Alice: {} Bob: {}", alice_compute, bob_compute);
}

fn test_sending_data() {
    let mut rng= rand::thread_rng();

    //first, generate random mod and base
    let g: i64 = rng.gen_range(2000000000..5000500000000);
    let x: i64 = rng.gen_range(50000000..10000500000);

    //Bob is sending to Alice
    let y: i64 = rng.gen_range(3..9);
    let com_sec: i64 = rng.gen_range(42..423);
    let data_file = fs::read("input_file.txt");

    let data: i64 = 44;

    //calculate g^xy mod p
    let inter = g^(com_sec*y) % x;
    let encrypted = data^inter;

    //This is now send to Alice in order to decrypt
    let inter2 = encrypted^inter;
    println!("{}", inter2);
}

fn main() {
    println!("Hello, world!");
}
