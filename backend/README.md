# Backend – Code VM / Game VM Program

This folder contains the on-chain program and supporting artifacts for the Game VM—a purpose-built virtual machine for reduced fees on Solana. This project is forked from the original Code VM repository, retaining core functionality while introducing customizations for gaming use cases.

---

## Overview

Game VM builds on the Code VM concept to enable:
- **Lower Transaction Fees:** Up to 95% lower fees and significantly reduced rent compared to standard Solana accounts.
- **Compressed Accounts:** Dormant accounts can be offloaded (compressed) off-chain, limiting rent costs.
- **On-Demand Decompression:** Accounts are decompressed automatically (via the Code app) or manually with on-chain logic.
- **Improved Gaming Performance:** Optimizations are tailored for game pool interactions and other gaming experiences on Solana.

---

## Repository Structure (Backend Only)

- **program/**  
  Contains the smart contract (on-chain program) written in Rust and built using the Anchor framework.  
  - The `src/` directory holds the program code (instructions, account declarations, etc.).
  - The program binary is generated in a Cargo workspace and includes the compressed IDL in an ELF section.

- **idl/**  
  Contains everything related to IDL generation and post-processing:
  - **Makefile:**  
    Manages building the dummy program using Anchor, running a custom discriminators update script, and moving the updated IDL files.
  - **src/scripts/update-discriminators.ts:**  
    A TypeScript script (run with Bun) that updates the generated IDL with human-readable discriminators from predefined mappings (preserving compatibility with code-vm API expectations).
  - **Anchor.toml, Cargo.toml:**  
    Configuration files for building the dummy program and generating the IDL.

- **api/** (if applicable)  
  Contains shared types and API definitions used by both the on-chain program and client applications.

---

## Getting Started

### Prerequisites

- **Rust & Cargo:** Ensure you have the latest stable release.
- **Solana CLI:** Install as per the [Solana docs](https://docs.solana.com/cli/install-solana-cli-tools).
- **Anchor Framework:** Version 0.31.1 (or the version specified in this repo) is required to build the program.
- **Node.js / npm & Bun.js:** Needed to run the IDL update script and manage frontend dependencies (if applicable).

### Building the Program & Generating the IDL

1. **Clean and Build the Dummy Program:**
   ```bash
   cd backend/idl/src
   cargo clean && anchor build --skip-lint
   ```

2. **Generate and Update the IDL:**
   From the `backend/idl` folder, run:
   ```bash
   make idl
   ```
   This command:
   - Uses Anchor to build the program.
   - Runs the `update-discriminators.ts` script to patch the generated IDL with human-readable discriminator values and default account types.
   - Updates the IDL file in `target/idl/code_vm.json` (and moves it to the appropriate locations).

3. **Copy to Frontend (Optional):**
   If using a frontend application, the Makefile also contains steps to copy the updated IDL to the frontend's `src/idl` folder. Ensure this step completes successfully.

### Deploying the Program

To deploy the smart contract onto your desired Solana cluster:
```bash
cd backend
anchor deploy
```

---

## Original Code VM Content

This repository was originally developed as Code VM. The core concepts remain unchanged:
- **Purpose:** Improve on-chain efficiency via compressed accounts and a custom IDL.
- **Compression/Decompression:** Accounts are compressed when dormant and decompressed on demand.
- **Interoperability:** The generated IDL allows for client generation using various languages and facilitates seamless integration with Solana explorers and frontends.

For more on the original design, please refer to the [original Code VM repository](https://github.com/code-payments/code-vm).

---

## Troubleshooting & Additional Notes

- **IDL Runtime Issues:** If you encounter errors like `Cannot read properties of undefined (reading 'size')` in the frontend, it is most likely due to missing or incomplete account `"type"` definitions in the IDL. The discriminators update script is designed to prevent this by assigning a default empty struct.  
- **Workspace Cleaning:** If you face build inconsistencies, consider running a workspace clean:
  ```bash
  cargo clean --workspace
  ```

- **Further Assistance:**  
  For questions or issues, please refer to the [issues section](https://github.com/code-payments/code-vm/issues) or join our community on [Discord](https://discord.gg/T8Tpj8DBFp).

---

## License

This project is licensed under the MIT License. See [LICENSE](../LICENSE) for details. 