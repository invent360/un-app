# External Dependencies

This directory contains vendored or stubbed external dependencies.

## ember-multichain (Stub)

The `ember-multichain` crate provides multi-chain wallet operations including:
- EVM client for Ethereum/World Mobile Chain
- SIWE (Sign-In With Ethereum) authentication
- Wallet import/creation/signing

**Current Status:** This is a minimal stub that provides the required types and constants for compilation. The stub allows `uno-admin` to build but wallet authentication features will not function.

### Setting Up the Real Crate

To enable full wallet functionality:

1. Clone the ember-multichain repository:
   ```bash
   git clone https://github.com/polkanight/ember-multichain.git deps/ember-multichain
   ```

2. Or use a git dependency in `uno-admin/Cargo.toml`:
   ```toml
   ember-multichain = { git = "https://github.com/polkanight/ember-multichain", features = ["evm"] }
   ```

## ember-fx (Vendored)

The ember-fx UI component library is vendored in `uno-app/deps/ember-fx/`. This includes:
- `ember-fx-components` - UI components (Table, Select, Button, etc.)
- `ember-fx-core` - Theme context and providers
- `ember-fx-icons` - Icon sets (Tabler, Flags)
- `ember-fx-utils` - Utility functions

Both `uno-app` and `uno-admin` reference this vendored copy.
