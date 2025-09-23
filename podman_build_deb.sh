#!/bin/bash
set -e

SCRIPT_DIR=$(dirname $0)
OUT_DIR=${SCRIPT_DIR}/target/deb

rm -rf $OUT_DIR
mkdir $OUT_DIR

podman build --tag covet-build-deb -f Dockerfile.build-deb
podman run --rm \
  --userns=keep-id \
  -v "${OUT_DIR}":/source/target/deb \
  covet-build-deb \
  /source/build_deb.sh
