rm -rf target/doc
cargo doc --no-deps
rm -rf docs
cp -r target/doc/reprypt ./docs