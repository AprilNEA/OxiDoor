.PHONY: license
license:
	addlicense -c "AprilNEA LLC" -l apache -s -y 2025 src/main.rs src/task.rs


server: 
	cargo build --release -p oxidoor-server

embedded:
	cargo build --release -p oxidoor-embedded

.PHONY: license
flash:
	espflash flash target/xtensa-esp32s3-espidf/debug/oxidoor