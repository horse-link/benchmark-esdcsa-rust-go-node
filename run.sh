echo "compiling rust"
cd rust && cargo build --release &>/dev/null &
echo "compiling go"
cd go && go build &>/dev/null &
echo "installing node dependencies"
cd node && yarn &>/dev/null &
echo "###########################################################"
echo "rust: $(./rust/target/release/benchmark-esdcsa) milliseconds"
echo "go: $(./go/benchmark-esdcsa-rust-go-node) milliseconds"
echo "node: $(cd node && node index.js) milliseconds"
echo "###########################################################"
