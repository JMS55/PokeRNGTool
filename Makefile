.PHONY: run
run: resources
	cd target/debug/ && cargo run --manifest-path ../../Cargo.toml

.PHONY: release
release:

.PHONY: clean
clean:
	cargo clean


.PHONY: resources
resources:
	glib-compile-resources --sourcedir=res res/PokeRNGTool.gresource.xml
	mkdir -p target/debug/
	mv res/PokeRNGTool.gresource target/debug/
