install:
	cargo install --path .

gen:
	icongen --src Icon-1024.png --dst icon

gen_custom:
	icongen --src Icon-1024.png --dst icon --icons icons.yml