
check:
	cargo check

test:
	cargo test

run:
	cargo run /home/dam/Téléchargementss 20

build:
	cargo build --release

build_win:
	cargo build --release --target x86_64-pc-windows-gnu