#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR/..
set -ex

if ! hash cargo-criterion 2>/dev/null; then
cargo install cargo-criterion
fi


#git_id=`git rev-parse --short HEAD`

#mkdir -p data/criterion
cargo criterion #--message-format=json > data/criterion/$git_id.json
open target/criterion/report/index.html
