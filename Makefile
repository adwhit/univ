SRCDIR = src
SDLDIR = lib/rust-sdl2
SDLLIB = $(SDLDIR)/build/lib/
FLAGS =  -O

all: univ

univ: $(SRCDIR)/main.rs 
	rustc $< -o $@ -L $(SDLLIB) $(FLAGS)

dep:$(SDLLIB)

test: $(SRCDIR)/tests.rs
	rm test -f
	rustc --test $< -o $@ -L $(SDLLIB) $(FLAGS)


$(SDLLIB): $(SDLDIR)
	cd $(SDLDIR); make

$(SDLDIR):
	git submodule init; git submodule update

depclean:
	rm -rf lib

clean:
	rm -f univ test

.PHONY: dep distclean clean test
