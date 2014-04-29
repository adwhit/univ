SDLDIR = "lib/rust-sdl2"
SDLLIB = "$(SDLDIR)/build/lib/libsdl2-79c1f430-0.0.1.rlib"
FLAGS = "-O"

all: univ

univ: univ.rs 
	rustc $< -o $@ -L $(SDLLIB) $(FLAGS)

dep:$(SDLLIB)

$(SDLLIB): $(SDLDIR)
	cd $(SDLDIR); make

$(SDLDIR):
	mkdir -p lib
	cd lib; git clone http://github.com/AngryLawyer/rust-sdl2

depclean:
	rm -rf lib

clean:
	rm univ

.PHONY: dep distclean clean
