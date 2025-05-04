PKGNAME = biblequote
BINDIR = /usr/local/bin
LICENSE_DIR = /usr/share/licenses/$(PKGNAME)

all: build

build:
	rustup toolchain install stable-x86_64-unknown-linux-gnu
	rustup run stable cargo build --release

install: build
	sudo install -Dm 755 target/release/$(PKGNAME) $(BINDIR)/$(PKGNAME)
	sudo install -Dm 644 LICENSE $(LICENSE_DIR)/LICENSE
	sudo install -Dm 644 README.md $(LICENSE_DIR)/README.md

uninstall:
	sudo rm -f $(BINDIR)/$(PKGNAME)
	sudo rm -rf $(LICENSE_DIR)

.PHONY: all build install uninstall
