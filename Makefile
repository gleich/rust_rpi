build:
	cargo build --release

deploy: build
	scp ./target/armv7-unknown-linux-gnueabihf/release/rust_rpi pi@mgdev.local:~