build:
	cargo build --release

install:
	cp "target/release/rip" "/usr/local/bin/rip"

uninstall:
	rm -f "/usr/local/bin/rip"
