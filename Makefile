PLUGINSDIR = $(DESTDIR)`pkg-config --variable=pluginsdir rofi`

all: target/release/libtchpad.so

clean:
	rm -fr Cargo.lock target

install: all
	mkdir -p $(PLUGINSDIR)
	cp -f target/release/libtchpad.so $(PLUGINSDIR)/tchpad.so

target/release/libtchpad.so:
	cargo build --release

uninstall:
	rm -f $(PLUGINSDIR)/tchpad.so

.PHONY: all clean install uninstall
