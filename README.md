# Learning WGPU

This is a personal repository for learning how **[WGPU](https://wgpu.rs/)
(24.0.1)** + **[Winit](https://github.com/rust-windowing/winit) (0.30.8)** and
how graphics rendering works, while following
[this](https://sotrh.github.io/learn-wgpu/) resource.

The project supports **cross-platform** compilation for **Windows**, **Linux**,
**MacOS** and **WebAssembly**, utilizing **WebGPU/WebGL** with
[trunk](https://trunkrs.dev/).

## Quickstart

```sh
# Clone the repository and open it
git clone https://github.com/Foxicution/wgpu-template
cd wgpu-template

# To run natively (Windows/MacOS/Linux)
cargo run

# To run on the web with WebAssembly (WASM)
# Add target wasm32-unknown-unknown for WASM builds
rustup target add wasm32-unknown-unknown

# Install trunk for WASM builds
cargo install --locked trunk

# With WebGL (for browsers that don't support WebGPU)
trunk serve --features webgl --open
# With WebGPU
trunk serve --open
```

For web builds, the app will be running on http://localhost:8080.

To check if your browser supports WebGPU go [here](https://webgpureport.org/).

# Resources

- [learn-wgpu](https://sotrh.github.io/learn-wgpu/)
