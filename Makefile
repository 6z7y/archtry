install:
	cargo build --release
	sudo install -m755 target/release/archtry /usr/bin/archtry

uninstall:
	sudo rm -f /usr/bin/archtry

clean:
	cargo clean

