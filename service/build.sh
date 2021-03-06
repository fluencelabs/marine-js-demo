#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# This script builds all subprojects and puts all created Wasm modules in one dir
mkdir -p artifacts
# cargo update
marine build --release

rm -f artifacts/*.wasm
cp target/wasm32-wasi/release/calc_service.wasm artifacts/
marine aqua artifacts/calc_service.wasm > artifacts/calc_service.aqua
