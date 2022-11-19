# ESDCSA Benchmark: Rust, Go, Node

This benchmark compares how fast the three languages can sign an EIP-191 message,
followed by signing an ETH transaction that includes that message.

The tests consists of signing the following message and transaction including
the signed message within the data field 1,000,000 times.

```solidity
keccak256(
    abi.encodePacked("The quick brown fox jumps over the lazy dog", 1337)
)
```
