# reth-custom-api-example

Example to extend Reth to include custom APIs.

This example adds a new custom api `eth_getGasUsedByBlock` to the node.

# Build
```
cargo run -- node --extend-eth-namespace --dev --http --http.api=eth,debug,reth --dev.block-time 12s --datadir /tmp/reth
```

or

```
cargo build --release
./target/release/reth-custom-api-example node --extend-eth-namespace --dev --http --http.api=eth,debug,reth --dev.block-time 12s --datadir /tmp/reth
```

# Test

```
cargo test
```