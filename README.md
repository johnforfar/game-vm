# Game VM

Game VM is a purpose-built virtual machine for reduced fees on Solana, designed for game challenge pools and interactive gaming experiences. Based off Code VM, this project inherits the core functionality from the original Code VM while integrating improvements and customizations for improved gaming performance and lower transaction costs.

![version](https://img.shields.io/badge/version-0.2.0-blue.svg?style=flat)
![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)

---

## Overview

The Code Virtual Machine (Code VM) was originally designed to facilitate highly efficient payments on Solana by working with compact account representations. Game VM adapts these concepts to the gaming context:
- **Reduced Fees:** Up to 95% lower transaction fees and 80% lower account rent compared to traditional Solana accounts.
- **Compressed Accounts:** Dormant accounts can be compressed off-chain, reducing on-chain rent costs to near zero.
- **On-Demand Decompression:** Accounts are decompressed automatically (via the Code app) or manually with support from public indexers.
- **Seamless Integration:** Designed for use by the Code app to provide millions of users with a seamless and low-cost gaming experience.

---

## Repository Structure

- **backend/**
  - **program/** – Contains the smart contract (on-chain program) written in Rust and built with Anchor.
  - **idl/** – Manages Interface Description Language (IDL) generation. It includes:
    - A custom Makefile that builds the dummy program with Anchor (using a specified feature set)
    - A script (`src/scripts/update-discriminators.ts`) powered by Bun.js to update account and instruction discriminators.
  - **api/** – (If applicable) API definitions and shared types for interacting with the smart contract.
- **frontend/** – Contains the React application that uses the generated IDL to interact with the deployed program on Solana.
- **Other Files**
  - The root README (this file) provides an overall overview.
  - Additional configuration files (e.g., Cargo.toml, Anchor.toml) are provided throughout the repository.

---

## Getting Started

### Prerequisites

- **Solana CLI:** Install as described in the [Solana docs](https://docs.solana.com/cli/install-solana-cli-tools).
- **Rust & Cargo:** Ensure the latest stable version is installed.
- **Anchor Framework:** Version 0.31.1 (or as specified by the repo) for building the program.
- **Node.js / npm:** Required for the frontend and IDL generation.
- **Bun.js:** Used to run the discriminator update script.
- **Vitest:** For testing the generated IDL.
- **(Optional) Git & Docker:** For version control and containerized deployment if needed.

### Installation

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/your-username/game-vm.git
   cd game-vm
   ```

2. **Install Backend Dependencies:**
   - Navigate into the IDL folder, install the npm dependencies, build with Anchor, and update the IDL:
     ```bash
     cd backend/idl/src
     npm install
     cargo clean
     anchor build --skip-lint
     cd ../      # back to backend/idl
     make idl
     ```
   - The Makefile in `backend/idl` runs the update script (`update-discriminators.ts`) and creates the final `code_vm.json` in `backend/idl` (which is then copied to the frontend).

3. **Setup the Frontend:**
   - Ensure the updated IDL is copied over:
     (This step is usually automatically triggered by the repository's Makefile.)
   - Install frontend dependencies:
     ```bash
     cd ../../frontend
     npm install
     npm run build
     npm start
     ```

4. **Run the Test Suite:**
   - Start a local Solana validator and create a keypair (e.g., `keypair-owner.json`).
   - Run the Vitest tests:
     ```bash
     vitest run ./backend/idl/src/tests/vm.test.ts --testTimeout 25000 --bail 1
     ```

---

## Development Workflow

### Backend (Smart Contracts & IDL)

- **Building the Program:**  
  The program uses Anchor to compile the smart contract. The IDL is generated during the build process:
  ```bash
  cd backend/idl/src
  cargo clean && anchor build --skip-lint
  cd ../
  make idl
  ```
- **IDL Discriminator Update:**  
  The `update-discriminators.ts` script modifies the generated IDL to include human-readable discriminators. Every account has a default type assigned if none exists, ensuring the Anchor client can compute the account layout without errors.
- **Deployment:**  
  Deploy your smart contract onto your desired Solana cluster:
  ```bash
  cd backend
  anchor deploy
  ```
  
### Frontend (React Application)

- **Using the IDL:**  
  The frontend imports the generated IDL (`frontend/src/idl/code_vm.json`) to create an Anchor client for interacting with the deployed smart contract.
- **Start the Application:**
  ```bash
  cd frontend
  npm run build
  npm start
  ```
- **Debugging:**  
  Check the browser console for errors. Ensure that the updated IDL contains proper `"type"` definitions for every account to avoid layout errors (such as missing `.size`).

---

## Original Code VM Content

This repository was forked from the original Code VM project. The core components of Code VM remain intact:

- **Purpose:** Maintain the original design of a compact on-chain virtual machine for payments on Solana.
- **Core Functionality:**  
  - Lowered transaction fees by using compressed on-chain accounts.
  - Delegated decompression when necessary.
- **References:**  
  Refer to the original repository for historical context and additional details:
  [Original Code VM Repository](https://github.com/code-payments/code-vm)

The Game VM extends these ideas for gaming use cases while preserving the innovative methods developed in Code VM.

---

## Community & Support

- **Discord:** [Join our Discord](https://discord.gg/T8Tpj8DBFp) for questions and community support.
- **Twitter:** Follow [@getcode](https://twitter.com/getcode) for updates.
- **Issues:** Please file any issues via GitHub.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to help out with this project.

---

## Contact

If you have further questions or need support, feel free to open an issue in this repository or contact the maintainers directly.
