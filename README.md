# strongholdTriangulationCLI
A simple CLI tool for triangulating strongholds in Minecraft using two Eye of Ender throws!

# Features
* Accurately computes the stronghold location from two throw co-ordinates and 'facing' angles
* Supports Linux, Windows , and MacOS
* Fast and lightweight (written in Rust)

# Installation

## Linux/MacOS

Install Rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Clone the repositroy and install:
```sh
git clone https://github.com/emmalt3/triangulateStronghold-CLI.git
cd triangulateStronghold-CLI
cargo install --path .
```

Run the CLI tool:
```sh
$ triangulateStronghold
```

## Windows

Install Rust: Download and install [Rustup](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe "Rustup Installer")

In a PowerShell terminal, run:
```ps
cargo install --git https://github.com/emmalt3/triangulateStronghold-CLI.git
```

Run the CLI tool:
```ps
$ triangulateStronghold
```

# Usage

## Command Line Input

When run, you will be prompted to enter two sets of Eye of Ender throw data:

1. Stand in your first location and throw an Eye of Ender
    * Record your X and Z co-ordinates, as well as the angle (theta) from the 'Facing' line on your F3 debug screen

2. Move to a new location and repeat

3. Enter the data when prompted by the CLI tool

Example Usage:
```sh
$ triangulateStronghold
input x1, z1, theta1:
> 3500 4600 60 
input x2, z2, theta2:
> 3192 5067 78
stronghold triangulated at: x = 2399, z = 5235
```

# Contributing

Want to improve this tool? Contributions are welcome!

1. Fork the repo
2. Make your changes
3. Submit a pull request

# License

This project is licensed under the [**Apache License 2.0**](LICENSE)
See the `LICENSE` file for details
