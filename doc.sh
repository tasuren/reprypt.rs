cargo doc --no-deps
rm -rf docs
cp -r target/doc ./docs
rm -rf target/doc