FRACTAL_GLYPH=../target/release/FractalGlyph

INPUT_CSV=$(wildcard data/*/*.csv)
THUMBNAILS=$(patsubst %.csv, %.png, $(INPUT_CSV))
THUMBNAILS+=$(patsubst %.csv, %-relative.png, $(INPUT_CSV))
THUMBNAILS+=$(patsubst %.csv, %-relative2.png, $(INPUT_CSV))



%.png: %.csv
	$(FRACTAL_GLYPH) --input $^ --kind fractal_pixels 

%-relative.png: %.csv
	$(FRACTAL_GLYPH) --input $^ --kind fractal_pixels_relative

%-relative2.png: %.csv
	$(FRACTAL_GLYPH) --input $^ --kind fractal_pixels_relative2

transform_all: $(THUMBNAILS)	

clean:
	-rm $(THUMBNAILS)