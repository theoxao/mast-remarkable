TARGET ?= armv7-unknown-linux-gnueabihf

build:
	cargo build --target=$(TARGET)


#DEVICE_IP ?= 10.11.99.1
DEVICE_IP ?= 192.168.31.200
DEVICE_HOST ?= root@$(DEVICE_IP)
deploy:
	ssh $(DEVICE_HOST) 'killall -q -9 mast_remarkable || true; systemctl stop xochitl || true'
	scp ./target/$(TARGET)/debug/mast_remarkable $(DEVICE_HOST):
	ssh $(DEVICE_HOST) 'RUST_BACKTRACE=full RUST_LOG=debug ./mast_remarkable'

docker:
	docker run \
		--rm \
		--user builder \
		-v $(shell pwd):/home/builder/libremarkable:rw \
		-v /Users/theo/.cargo/registry:/home/builder/.cargo/registry \
		-w /home/builder/libremarkable \
		rust-build-remarkable:latest \
		cargo build --target=$(TARGET)

run: docker deploy

start-xochitl:
	ssh $(DEVICE_HOST) 'killall -q -9 mast_remarkable || true; systemctl start xochitl'
