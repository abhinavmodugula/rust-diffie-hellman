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
