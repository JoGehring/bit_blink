# Contributor:
# Maintainer:
pkgname=bit_blink
pkgver=0.1
pkgrel=1
pkgdesc="This is a test package"
url="https://github.com/JoGehring/bit_blink"
arch="all"
license="Apache 2.0"
makedepends="
	cargo
	cargo-auditable
	cargo-gra
	libadwaita-dev
	"
checkdepends=""
install=""
# subpackages="$pkgname-dev $pkgname-doc"
source="https://github.com/JoGehring/bit_blink/archive/refs/tags/package_testing.tar.gz"
builddir="$srcdir/bit_blink-package_testing"

prepare() {
	default_prepare

	cargo fetch --target="aarch64-alpine-linux-musl" 
}

build() {
	cargo auditable build --frozen --release
}

check() {
	cargo test --frozen
}

package() {
	install -Dm755 target/release/bit_blink -t "$pkgdir"/usr/bin
}

sha512sums="
08200d9d9936608fd5ec06e8a5aa11f3a17a6b9614ad493e8180972a13eced9e2cd7d7093add72d880a6242b884ee094854f4ff149e8bed3de634b890fcc7fbd  package_testing.tar.gz
"
