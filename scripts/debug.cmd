@echo off
:: cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console" && echo Initializing... && target\debug\kitty_paw.exe
cargo run --target x86_64-target.json 