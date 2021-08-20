# Maintainer: PryosCode <info@pryoscode.net>
pkgname="toru"
pkgver="0.1.0"
pkgrel="1"
pkgdesc="Pacman wrapper with AUR support."
arch=("x86_64")
license=("Apache-2.0")
depends=(
    "autoconf"
    "automake"
    "binutils"
    "bison"
    "fakeroot"
    "file"
    "findutils"
    "flex"
    "gawk"
    "gcc"
    "gettext"
    "grep"
    "groff"
    "gzip"
    "libtool"
    "m4"
    "make"
    "pacman"
    "patch"
    "pkgconfig"
    "sed"
    "sudo"
    "texinfo"
    "which"
    "git"
)
makedepends=(
    "cargo"
)
source=("https://github.com/PryosCode/toru/archive/refs/tags/v${pkgver}.tar.gz")
sha512sums=("SKIP")

build() {
    cd $pkgname-$pkgver
    cargo build --release
}

package() {
    mkdir -p $pkgdir/usr/bin/
    mv $pkgname-$pkgver/target/release/toru $pkgdir/usr/bin/
}