build-pages:
	cd pong && cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web --out-dir ./docs/ --out-name "pong" ./pong/target/wasm32-unknown-unknown/release/pong.wasm
	wasm-opt -O -ol 100 -s 100 -o docs/pong_bg2.wasm docs/pong_bg.wasm
	mv docs/pong_bg2.wasm docs/pong_bg.wasm
	cp -r pong/assets docs/

