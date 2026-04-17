# Prepare Ubuntu-24.04
* sudo apt update && sudo apt upgrade -y
* sudo apt install build-essential curl -y
* curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
* rustup target add wasm32-unknown-unknown
* cargo install wasm-pack
* cargo install http-server

# Build project
* wasm-pack build --target web --out-dir ./pkg

# Local Running
* python3 -m http.server 8080