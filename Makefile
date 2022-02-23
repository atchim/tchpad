PLUGIN = tchpad
PLUGINSDIR = $(DESTDIR)`pkg-config --variable=pluginsdir rofi`
RELEASE = target/release/lib$(PLUGIN).so

all: $(RELEASE)

$(RELEASE):
	cargo build --release

clean:
	rm -fr target

install: $(RELEASE)
	mkdir -p $(PLUGINSDIR)
	cp -f $< $(PLUGINSDIR)/$(PLUGIN).so

uninstall:
	rm -f $(PLUGINSDIR)/$(PLUGIN).so

.PHONY: all clean install uninstall
