#!/usr/bin/env bash

set -e
DIR=$( dirname $(realpath "$0") )
source $DIR/pid.sh

cd $DIR/../..
npm run prepare
DENO_DEV=1 exec deno run --import-map=import.map.json --unstable -A ${!#}

