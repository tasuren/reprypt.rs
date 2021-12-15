rm -rf target/doc
cargo doc --no-deps
cp -r target/doc/reprypt ./doc