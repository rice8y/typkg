# TyPkg

**TyPkg** is a CLI tool to install Typst packages from GitHub as **local packages**.  

Installed packages can be used in Typst with the local import syntax:

```typst
#import "@local/<package>:<version>"
```

> [!NOTE] 
> Typst also supports official preview packages with `@preview/<package>:<version>`
TyPkg does not interact with the official registry. It is intended for packages that are not published officially and need to be installed locally.

Traditionally, installing a local package required cloning the repository and copying it into a directory where Typst could see it, sometimes using a `justfile` with `just install`. TyPkg automates this process.

## Installation

You can install TyPkg directly from GitHub using Cargo:

```bash
cargo install --git https://github.com/rice8y/typkg.git
```

Alternatively, you can clone the repository and build manually:

```bash
git clone https://github.com/rice8y/typkg.git
cd typkg
cargo build --release
chmod +x target/release/typkg
mv target/release/typkg ~/.local/bin/typkg
```

Make sure `~/.local/bin` is in your PATH.

## Usage

```bash
# Install a Typst package from GitHub
typkg install https://github.com/username/typst-package

# List installed packages
typkg list

# Remove a package
typkg clean <package_name> <version>
```

## Commands

| Command   | Description                                         |
| --------- | --------------------------------------------------- |
| `install` | Install a Typst package from GitHub or local path   |
| `list`    | Show all locally installed Typst packages           |
| `clean`   | Remove a specific local package by name and version |

## License

This project is distributed under the MIT License. See [LICENSE](LICENSE).