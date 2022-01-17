build:
	cargo build --target=armv7-unknown-linux-gnueabihf

deploy: build
	scp ./target/armv7-unknown-linux-gnueabihf/debug/rust_rpi pi@mgdev.local:~