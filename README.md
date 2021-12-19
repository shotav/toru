<p align="center">
    <a href="https://github.com/PryosCode/toru/tags"><img alt="Version" src="https://img.shields.io/github/v/release/PryosCode/toru?label=Version"></a>
    <a href="https://github.com/PryosCode/toru/actions/workflows/build.yml"><img alt="Build" src="https://github.com/PryosCode/toru/actions/workflows/build.yml/badge.svg"></a>
    <a href="https://github.com/PryosCode/toru/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/PryosCode/toru?label=License"></a>
</p>

# <a href="https://github.com/PryosCode/toru/blob/master/img/pacman.png"><img src="https://github.com/PryosCode/toru/raw/master/img/pacman.png" alt="Pacman" width="30" height="auto"></a> toru

Pacman wrapper with AUR support.

## Installation

```bash
pacman -S git base-devel
git clone https://aur.archlinux.org/toru.git
cd toru
makepkg -si
```

## Usage

```bash
USAGE:
    toru <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clean      Clean cache.
    install    Install the specified packages.
    remove     Remove the specified packages.
    update     Update all packages.
```

## License

[Apache License 2.0](https://github.com/PryosCode/toru/blob/master/LICENSE)