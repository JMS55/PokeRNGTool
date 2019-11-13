run: resources
	mkdir -p target/debug/
	cd target/debug/ && cargo run --manifest-path ../../Cargo.toml

release:

clean:
	cargo clean


resources: res/PokeRNGTool.gresource.xml res/ui/main.ui
	glib-compile-resources --sourcedir=res res/PokeRNGTool.gresource.xml
	mkdir -p target/debug/
	mv res/PokeRNGTool.gresource target/debug/
