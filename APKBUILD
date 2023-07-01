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
source="https://github.com/JoGehring/bit_blink/archive/refs/tags/$pkgver.tar.gz"
builddir="$srcdir/bit_blink-$pkgver"

prepare() {
	default_prepare

	cargo fetch --target="aarch64-alpine-linux-musl" 
}

build() {
	cargo auditable build --frozen --release
}

check() {
	#cargo test --frozen --test-threads=1
}

package() {
	install -Dm755 target/release/bit_blink -t "$pkgdir"/usr/bin
	#cp bit_blink.desktop /usr/share/applications/
	#cp bitBlinkIcon.png /usr/share/icons/
	
}

sha512sums=""
