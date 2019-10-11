default:
	cargo build --release
	strip ./target/release/cpu-stats-logger
	cp ./target/release/cpu-stats-logger .
