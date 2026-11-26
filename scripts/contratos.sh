#!/usr/bin/env bash
# Compila, prueba y opera los contratos dentro de Docker.
# No hace falta instalar Rust ni el CLI de Stellar en la máquina.
#
# Uso, desde la raíz del repositorio:
#   bash scripts/contratos.sh test                 corre todas las pruebas
#   bash scripts/contratos.sh build                compila los .wasm en wasm/
#   bash scripts/contratos.sh stellar <args...>    ejecuta el CLI de Stellar
#   bash scripts/contratos.sh script <archivo>     ejecuta un script con el CLI disponible
#
# Las claves de las cuentas de prueba quedan en el volumen de Docker
# "fidedigno-stellar", fuera del repositorio.
set -euo pipefail

CLI_VERSION=30.0.1
raiz="$(pwd -W 2>/dev/null || pwd)"
export MSYS_NO_PATHCONV=1

docker run --rm \
  -v "${raiz}:/work" \
  -v fidedigno-cargo:/usr/local/cargo/registry \
  -v fidedigno-target:/cache \
  -v fidedigno-bin:/opt/bin \
  -v fidedigno-stellar:/root/.config/stellar \
  -e CARGO_TARGET_DIR=/cache \
  -e CLI_VERSION="${CLI_VERSION}" \
  -w /work \
  rust:1 bash -c '
    set -e
    export PATH="/opt/bin:${PATH}"
    if [ ! -x /opt/bin/stellar ]; then
      curl -sSfL "https://github.com/stellar/stellar-cli/releases/download/v${CLI_VERSION}/stellar-cli-${CLI_VERSION}-x86_64-unknown-linux-gnu.tar.gz" \
        | tar -xz -C /opt/bin stellar
    fi
    accion="${1:?uso: contratos.sh test|build|stellar|script <args...>}"
    shift
    if [ "${accion}" != "test" ]; then
      # El CLI de Stellar necesita estas librerías, que la imagen no trae.
      apt-get update -qq >/dev/null && apt-get install -y -qq libdbus-1-3 libudev1 >/dev/null
    fi
    case "${accion}" in
      test)
        cargo test "$@" ;;
      build)
        rustup target add wasm32v1-none >/dev/null 2>&1
        stellar contract build
        mkdir -p wasm
        cp /cache/wasm32v1-none/release/*.wasm wasm/
        sha256sum wasm/*.wasm ;;
      stellar)
        stellar "$@" ;;
      script)
        bash "$@" ;;
      *)
        echo "acción desconocida: ${accion}" >&2
        exit 2 ;;
    esac
  ' _ "$@"
