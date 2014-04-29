SDLDIR = "lib/rust-sdl"
SDLSRC = "$(SDLDIR)/src/sdl/lib.rs"
SDLLIB = "libsdl-e351513a-0.3.2.rlib" 

dep:$(SDLDIR)/$(SDLLIB)

$(SDLDIR)/$(SDLLIB): lib/rust-sdl
	rustc $(SDLSRC) -O --out-dir $(SDLDIR)

lib/rust-sdl: 
	mkdir -p lib
	cd lib; git clone http://github.com/brson/rust-sdl; \
		cd rust-sdl; git checkout 804adf6

depclean:
	rm -rf lib

.PHONY: dep distclean
