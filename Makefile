SRCDIR = src
LIBDIR = lib
TOMLDIR = $(LIBDIR)/rust-toml
TOMLLIBDIR = $(TOMLDIR)/lib
TOMLLIB = $(TOMLLIBDIR)/librust-toml-9138e0a4-toml.rlib
SDLDIR = $(LIBDIR)/rust-sdl2
SDLLIBDIR = $(SDLDIR)/build/lib/
SDLLIB = $(SDLLIBDIR)/libsdl2-79c1f430-0.0.1.rlib
FLAGS =  -O

all: univ

univ: $(SRCDIR)/main.rs 
	rustc $< -o $@ -L$(SDLLIBDIR) -L$(TOMLLIBDIR) $(FLAGS)

dep: $(SDLLIB) $(TOMLLIB)

test: $(SRCDIR)/tests.rs
	rustc --test $< -o $@ -L $(SDLLIB) $(FLAGS)


$(SDLLIB): $(SDLDIR) 
	cd $(SDLDIR); make

$(TOMLLIB): $(TOMLDIR)
	cd $(TOMLDIR); make

$(SDLDIR):
	git submodule init; git submodule update

depclean:
	rm -rf lib

clean:
	rm -f univ test

.PHONY: all dep distclean clean univ test
