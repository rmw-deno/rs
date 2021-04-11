#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

deno run --allow-read --allow-env --unstable deno/test.ts

