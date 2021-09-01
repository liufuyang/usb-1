#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=root@venus.local
readonly TARGET_PATH=/home/root/usb-qucc
readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/usb-qucc

cargo build --target=${TARGET_ARCH} --release 
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}