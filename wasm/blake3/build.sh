#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

ssvmup build --release --target deno
mv pkg/*.js ../
mv pkg/*.wasm ../
