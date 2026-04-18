# Prepare Ubuntu-24.04
* sudo apt update && sudo apt upgrade -y
* sudo apt install build-essential curl -y
* curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
* rustup target add wasm32-unknown-unknown
* cargo install wasm-pack
* cargo install trunk

# Build and running
* trunk serve --port 8080
