# Easy Commit Messages

This software is designed to make your life easier by writing your commit messages for you

# Examples

```
easycommitmessage
>>> fixed identical bug
easycommitmessage
>>> removed feline feature
easycommitmessage
>>> adds charming bug
```

# Installation

Download the latest binary from the [releases](https://github.com/lyqio/easy-commit-messages/releases) and then you will be able to run the .exe file to use the program.

# Build From Source

First make sure you have rust installed with cargo

```bash
cargo --version
```

Clone this repo and build:

```bash
git clone https://github.com/lyqio/easy-commit-messages.git
cd easy-commit-messages
cargo build
```
The resulting exe file should be located in the directory

```bash
target/debug/easy-commit-messages.exe
```

Or use a release build
```bash
cargo build --release
```

Where the resulting file is found at
```bash
target/release/easy-commit-messages.exe
```
