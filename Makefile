# Makefile for building WASM version of this code.

all: out/jbob.js out/index.html

# Use cargo to build our bare wasm module. We mark this as ".PHONY" to ensure
# that even if the output file exists, cargo always gets called to do its own
# dependency tracking. (Which it's good at!)
target/wasm32-unknown-unknown/release/jbob.wasm:
	cargo build --target wasm32-unknown-unknown --release
.PHONY: target/wasm32-unknown-unknown/release/jbob.wasm

# Use wasm-bindgen to wrap our bare wasm module into something callable from JS.
out/jbob.js: target/wasm32-unknown-unknown/release/jbob.wasm out
	wasm-bindgen $< --out-dir out --no-modules

# Copy our demo HTML file to our output.
out/index.html: src/static/index.html out
	cp $< $@

# Make sure we have our output directory.
out:
	mkdir -p out

# Clean up all build output.
clean:
	cargo clean
	rm -rf out
.PHONY: clean
