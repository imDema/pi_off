## Install

### Debian based
+ Get the latest .deb
+ Install using `sudo dpkg -i [pi_off_version_armhf.deb]`

(You may need to run `sudo sytemctl enable pi_off.service && sudo systemctl start pi_off.service`)

## Compile
+ Either clone the repo or download the source from the latest release
+ Build using `cargo build --release`
+ Run the install script `sudo ./install`

### Optional: build deb package
+ Install cargo-deb `cargo install cargo-deb`
+ Create the package `cargo deb --target armv7-unknown-linux-gnueabihf`

### Cross-compile
+ Install the armhf toolchain `rustup target install armv7-unknown-linux-gnueabihf`
+ Install gcc for armhf (`sudo apt install gcc-arm-linux-gnueabihf` on Debian / Ubuntu)
+ Follow the compile instruction