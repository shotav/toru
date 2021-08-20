# Maintainer: PryosCode <info@pryoscode.net>
pkgname="toru"
pkgver="0.1.0"
pkgrel="1"
pkgdesc="Pacman wrapper with AUR support."
arch=("x86_64")
license=("Apache-2.0")
depends("git", "base-devel", "pacman", "sudo")
makedepends("cargo")
source=("https://github.com/PryosCode/toru/archive/refs/tags/v{pkgver}.tar.gz")
sha512sums=("SKIP")

prepare() {
}

build() {
    cargo build --release
}

package() {
    mv target/release/toru $pkgdir
}