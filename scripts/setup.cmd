@echo off
rustup target add thumbv7em-none-eabihf
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc

cargo install bootimage
rustup component add llvm-tools-preview