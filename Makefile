TARGET ?= armv7-unknown-linux-gnueabihf

build:
	cargo build --release --target=$(TARGET)


DEVICE_IP ?= 192.168.2.189
DEVICE_HOST ?= root@$(DEVICE_IP)
deploy:
	ssh $(DEVICE_HOST) 'killall -q -9 mast_remarkable || true; systemctl stop xochitl || true'
	scp ./target/$(TARGET)/release/mast_remarkable $(DEVICE_HOST):
	ssh $(DEVICE_HOST) 'RUST_BACKTRACE=full RUST_LOG=debug ./mast_remarkable'


run: build deploy

start-xochitl:
	ssh $(DEVICE_HOST) 'killall -q -9 mast_remarkable || true; systemctl start xochitl'
