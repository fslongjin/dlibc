all:
	rustup default nightly
	cargo +nightly build --release --target src/arch/x86_64/x86_64-unknown-dragonos.json