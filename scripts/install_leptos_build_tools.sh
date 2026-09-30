#!/bin/sh
set -eu

case "$(dpkg --print-architecture)" in
    arm64)
        leptos_arch=aarch64
        leptos_sha=43de7e1b6adc00dfc14504a101e1a9da066e28187b1bbe30d93bdb9632720eb4
        bindgen_arch=aarch64-unknown-linux-gnu
        bindgen_sha=1797c349a0f45d30946e8986b184135e12839ab576c9a04203e85febfc5dcb35
        ;;
    amd64)
        leptos_arch=x86_64
        leptos_sha=fbd1013f9543db0cde37dbf7bf1661d3d92b499d6337165bd625da279c82e022
        bindgen_arch=x86_64-unknown-linux-musl
        bindgen_sha=82d12bb940e2d4e72e0d5605387fc1b8ca179044e012b620f0ce4e7440e8320
        ;;
    *) echo 'Unsupported Docker build architecture' >&2; exit 1 ;;
esac

leptos_archive="cargo-leptos-${leptos_arch}-unknown-linux-gnu.tar.gz"
curl --fail --silent --show-error --location \
    "https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.6/${leptos_archive}" \
    --output "/tmp/${leptos_archive}"
printf '%s  %s\n' "$leptos_sha" "/tmp/${leptos_archive}" | sha256sum --check --status
tar -xzf "/tmp/${leptos_archive}" -C /tmp
install -m 0755 "/tmp/cargo-leptos-${leptos_arch}-unknown-linux-gnu/cargo-leptos" /usr/local/bin/cargo-leptos

bindgen_archive="wasm-bindgen-0.2.129-${bindgen_arch}.tar.gz"
curl --fail --silent --show-error --location \
    "https://github.com/wasm-bindgen/wasm-bindgen/releases/download/0.2.129/${bindgen_archive}" \
    --output "/tmp/${bindgen_archive}"
printf '%s  %s\n' "$bindgen_sha" "/tmp/${bindgen_archive}" | sha256sum --check --status
tar -xzf "/tmp/${bindgen_archive}" -C /tmp
install -m 0755 "/tmp/wasm-bindgen-0.2.129-${bindgen_arch}/wasm-bindgen" /usr/local/bin/wasm-bindgen

cargo-leptos --version
wasm-bindgen --version
