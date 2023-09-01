# reth-custom-api-example

Example to extend Reth to include custom APIs.

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
TODO, add test in Rust

```javascript
const { ethers } = require("ethers");
const { parseUnits } = require("ethers/lib/utils");

const main = async () => {
  const provider = new ethers.providers.JsonRpcProvider(
    "http://127.0.0.1:8545",
    1337
  );
  const wallet = new ethers.Wallet(
    "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"
  ).connect(provider);

  const bribe = parseUnits("0.01");
  const gasLimit = 21000;
  const ethPerGas = bribe.div(gasLimit);

  const walletAddr = await wallet.getAddress();

  const txReq = {
    from: walletAddr,
    to: walletAddr,
    gasLimit: gasLimit,
    maxFeePerGas: ethPerGas,
    maxPriorityFeePerGas: ethPerGas,
    type: 2,
    chainId: 1,
  };

  let tx;
  for (let i = 0; i < 6; i++) {
    tx = await wallet.sendTransaction({
      gasPrice: parseUnits("15", 9),
      to: wallet.address,
    });
  }
  await tx.wait();

  const bn = await provider.getBlockNumber();

  const resp = await provider.send("eth_getGasUsedByBlock", [
    "0x" + bn.toString(16),
  ]);
  console.log("gasUsed", resp);
}
```