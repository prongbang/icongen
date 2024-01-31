install:
	cargo install --path .

gen:
	icongen --src Icon-1024.png --dst icon

gen_custom:
	icongen --src Icon-1024.png --dst icon --icons icons.yml
	
# make build_macos version=0.1.0
build_macos:
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	make zip_macos_x86_64 version=$(version)
	make zip_macos_arm64 version=$(version)

# make zip_macos_x86_64 version=0.1.0
zip_macos_x86_64:
	cd target/x86_64-apple-darwin/release && \
	tar -zcvf $(version)_Darwin_x86_64.tar.gz icongen && \
	cd ../../../

# make zip_macos_arm64 version=0.1.0
zip_macos_arm64:
	cd target/aarch64-apple-darwin/release && \
	tar -zcvf $(version)_Darwin_arm64.tar.gz icongen && \
	cd ../../../

# make build_macos version=0.1.0
build_macos_release:
	make build_macos version=0.1.0