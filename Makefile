js:
	node ./node/index.js

rs: 
	cargo run --manifest-path ./rust/Cargo.toml

rs-bin:
	./rust/target/release/cody

benchmark:
	hyperfine --export-markdown benchmark.md --min-runs 20 --warmup 3 'make js' 'make rs' 'make rs-bin'
