cargo doc --no-deps
rm -rf docs
cp -r target/doc ./docs
rm -rf target/doc
cp redirect.html docs/index.html