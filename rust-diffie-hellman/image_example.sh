if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <image_name>"
    exit 1
fi

secret="secret"

./target/release/rust-diffie-hellman pick_random $secret
./target/release/rust-diffie-hellman encrypt $1 $secret
./target/release/rust-diffie-hellman decrypt $1 $secret

