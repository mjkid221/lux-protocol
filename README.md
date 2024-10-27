# Compressed Program Template

This repo is based on the ZK Compression template initialized via `light init testprogram
`. It adds TS based tests to help the reader understand how to write tests for a Light program in Anchor.

## Prerequisites

- [Light CLI](https://github.com/Lightprotocol/light-protocol/tree/03b17ab48b6292a1abd1c2a8dac0a2b7d49e6e30/cli) must be installed.
- Ensure you have Anchor and Solana CLI tools installed.

## Setup

1. Generate a new Anchor key:
   ```
   anchor keys list
   ```

2. Replace the generated key in `lib.rs` and `Anchor.toml`.

3. Build the program:
   ```
   anchor build
   ```

## Testing

This project supports both Rust (tokio) and TypeScript test suites.

### Rust Tests

Run the native Rust tests that come with the Anchor ZK compression template:

```
yarn test:tokio
```

### TypeScript Tests 
Author: @mjkid221

The newly written TypeScript tests more or less provider the same test coverage as the standard native tokio tests provided by the Anchor ZK compression template. To run the full TypeScript test suite:

```
yarn test:ts:full-setup
```

This command will:
1. Start the Light test-validator locally
2. Deploy the program
3. Run the TypeScript tests

For more granular control, you can use these scripts:

- `yarn lightv`: Start the Light test-validator
- `yarn depl`: Deploy the program
- `yarn test:ts:no-setup`: Run TypeScript tests without setup steps

## Important Notes

- The Light cargo dependencies in `Cargo.toml` are pinned to revision `ac34e7223423c2e1022554db30c7748714962cbb` for stability.

## Troubleshooting

If you encounter any connection issues with the light test validator setup or other error on port 3001:

1. Find the process ID:
   ```
   lsof -i:3001
   ```

2. Kill the process:
   ```
   kill <pid>
   ```