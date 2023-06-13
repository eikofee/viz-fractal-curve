run:
	cargo build --release

	rm hillbertInfo.xybary; ./target/release/FractalGlyph --input hillbertInfo.csv --kind  barycenter_points && head hillbertInfo.xybary
	python vis_points.py hillbertInfo.xybary hillbertInfo.csv

	rm head hillbertInfo.xy; ./target/release/FractalGlyph --input hillbertInfo.csv --kind  fractal_points && head hillbertInfo.xy
	python vis_points.py hillbertInfo.xy hillbertInfo.csv
	
	
	./target/release/FractalGlyph --input hillbertInfo.csv
	xdg-open hillbertInfo.png
