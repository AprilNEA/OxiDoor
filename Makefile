server: 
	cargo build --release -p oxidoor-server

embedded:
	cargo build --release -p oxidoor-embedded

.PHONY: license
flash:
	espflash flash target/xtensa-esp32s3-espidf/debug/oxidoor