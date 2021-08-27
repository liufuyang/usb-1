Learing about USB

https://www.youtube.com/watch?v=YV6Qa5vHAV8&t=601s

```
# In order to build a runnable file on raspberry pi
# https://pixelspark.nl/2020/cross-compiling-rust-programs-for-a-raspberry-pi-from-macos
# https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050
# https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/

brew install arm-none-linux-gnueabihf-binutils
rustup target add arm-unknown-linux-musleabi
touch .cargo/config

stty -f /dev/tty.debug-console -a
```