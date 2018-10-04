# Makefile for building WASM version of this code.

all: webpack

# Install our JS dependencies using YARN.
yarn:
	yarn install
.PHONY: yarn

# Use cargo to build our bare wasm module. We mark this as ".PHONY" to ensure
# that even if the output file exists, cargo always gets called to do its own
# dependency tracking. (Which it's good at!)
target/wasm32-unknown-unknown/release/jbob.wasm:
	cargo build --target wasm32-unknown-unknown --release
.PHONY: target/wasm32-unknown-unknown/release/jbob.wasm

# Use wasm-bindgen to wrap our bare wasm module into something callable from JS.
site/jbob_bg.wasm: target/wasm32-unknown-unknown/release/jbob.wasm
	wasm-bindgen --out-dir site --browser $<

# Compile our JS/TS code into an actual bundle.
webpack: site/jbob_bg.wasm
	node_modules/.bin/webpack-cli
.PHONY: webpack

# Serve up our rendered site.
serve:
	(cd site; python -m SimpleHTTPServer)

# Profile (dethm.align/align) with our interpreter.
profile:
	cargo build --release --example dethm-align-align
	cargo profiler callgrind --release --bin target/release/examples/dethm-align-align
.PHONY: profile

# Clean up all build output.
clean:
	cargo clean
	rm -rf out
.PHONY: clean
