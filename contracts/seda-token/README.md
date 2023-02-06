SEDA Fungible Token (FT)
===================

Basic implementation of a [Fungible Token] contract for the SEDA protocol which uses [near-contract-standards].

  [Fungible Token]: https://nomicon.io/Standards/Tokens/FungibleTokenCore.html
  [near-contract-standards]: https://github.com/near/near-sdk-rs/tree/master/near-contract-standards

NOTES:
 - The maximum balance value is limited by U128 (2**128 - 1).
 - JSON calls should pass U128 as a base-10 string. E.g. "100".
 - This does not include escrow functionality, as `ft_transfer_call` provides a superior approach. An escrow system can, of course, be added as a separate contract.

## Building
To build run in root:
```make build-contracts
```

## Testing
To test run:
```bash
cargo test --package seda-token -- --nocapture
```
