#!/bin/sh
set -e

REPO="obikosato/jcal"
INSTALL_DIR="/usr/local/bin"

# OS/arch判定
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
	Darwin) os="apple-darwin" ;;
	Linux) os="unknown-linux-gnu" ;;
	*)
		echo "Unsupported OS: ${OS}"
		exit 1
		;;
esac

case "${ARCH}" in
	arm64 | aarch64) arch="aarch64" ;;
	x86_64) arch="x86_64" ;;
	*)
		echo "Unsupported architecture: ${ARCH}"
		exit 1
		;;
esac

TARGET="${arch}-${os}"
URL="https://github.com/${REPO}/releases/latest/download/jcal-${TARGET}.tar.gz"

TMPDIR="$(mktemp -d)"
trap 'rm -rf "${TMPDIR}"' EXIT

echo "Downloading jcal for ${TARGET}..."
curl -fsSL "${URL}" | tar xz -C "${TMPDIR}"

echo "Installing to ${INSTALL_DIR}/jcal..."
if [ -w "${INSTALL_DIR}" ]; then
	mv "${TMPDIR}/jcal" "${INSTALL_DIR}/jcal"
else
	sudo mv "${TMPDIR}/jcal" "${INSTALL_DIR}/jcal"
fi

echo "Done! Run 'jcal' to start."
