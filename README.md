# ESDCSA Benchmark: Rust, Go, Node, Python and C#

This benchmark compares how fast the five languages can sign an EIP-191 message,
followed by signing an ETH transaction that includes that message.

The tests consist of signing the following message and transaction including
the signed message within the data field 100,000 times.

```solidity
keccak256(
    abi.encodePacked("The quick brown fox jumps over the lazy dog", 1337)
)
```

## Results 

The results were obtained on a machine with the following specs:

- Intel i5-9600K (6) @ 4.6 GHz
- 16 GB of RAM

|     | Rust | Go   | Node   |
|-----|------|------|--------|
| ms  | 2430 | 2344 | 156399 |
