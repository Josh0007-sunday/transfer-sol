# Solana SOL Transfer Example

This example demonstrates how to transfer SOL between system accounts using the Anchor framework.

## Description

A minimal example showing how to:
1. Initialize a program
2. Transfer SOL between system accounts
3. Properly handle errors and account validation

## Quick Start

1. Install dependencies:
```bash
yarn install
```

2. Build the program:
```bash
anchor build
```

3. Update the program ID:
```bash
solana-keygen new -o target/deploy/sol_transfer-keypair.json
anchor keys list
# Update the program ID in lib.rs and Anchor.toml
```

4. Run tests:
```bash
anchor test
```

## Program Structure
- `programs/sol-transfer/src/lib.rs`: Main program logic
- `tests/sol-transfer.ts`: Test cases
- `Anchor.toml`: Program configuration

## License
MIT