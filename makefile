build-cli:
	cd cli && cargo build --release

run-cli-big:
	cd cli && cargo run AddFile ../file/big_4k.exr

run-cli-small:
	cd cli && cargo run AddFile ../file/small.txt