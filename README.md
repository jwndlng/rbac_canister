# Role-Based Access Control (RBAC) for the Internet Computer

This project provides a Role-Based Access Control (RBAC) implementation for canisters running on the Internet Computer, written in Rust. 

## Overview

This RBAC implementation features a role-based authorization system, with different roles having different levels of access and permissions. 

There are three predefined roles:
1. **Admin**: The highest authority, this role has all permissions including managing roles for all canisters.
2. **Manager**: Has broad control but cannot manage other Admin roles.
3. **Viewer**: Can only call functions that do not modify the canister's state.

## Design Decisions

- The RBAC system is designed to apply uniform roles across all canisters it controls. This means that the permissions of a particular role are consistent across all canisters, which ensures uniformity of access rights.
- The permission mapping provided by the RBAC canister is universal, and is applicable to all canisters that have the RBAC canister set as their controller. This design choice simplifies code and management, and offers a broad level of control over multiple canisters. However, this comes with the trade-off of reduced granularity in permission control on a per-canister basis, which might be less ideal for complex use-cases requiring fine-tuned permissions.

## Getting Started

To run this project, you will need `dfx`, the command-line tool for developing dapps on the Internet Computer. 

### Prerequisites

- Install `dfx` by running `sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"`
- Install Rust via [rustup](https://rustup.rs/)

### Building the Project

- Run `dfx start --background` to start the local network.
- Run `dfx deploy` to compile and deploy the canister.

## Usage

Refer to the provided Rust code in `src/` for usage.

## Roadmap/To-dos

- The `install_code` and `uninstall_code` functions are under development. Upon completion, they will provide additional control over the canisters.
- A significant enhancement will be support for asset canisters, allowing this RBAC system to manage access to various kinds of digital assets.
- The RBAC canister is designed with CI/CD integrations in mind. A future update will include a demonstration of a GitHub Action deployment using the RBAC canister.
- Review if its possible to use `dfx` to WASM Code and format, so it can be used in a canister call.

## Contributing

We appreciate your contributions! Please submit a pull request with your improvements.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources & Acknowledgements

- Roman Kashitsyn for writing great documentation on his blog [mmapped.blog](https://mmapped.blog/).
- Developer documentation on [internetcomputer.org](https://internetcomputer.org)
