
build-cli:
	cd cli && cargo build --release

run-cli:
	cd cli && cargo run AddFile ../d.txt