#!/usr/bin/env bash


git pull

./clippy.sh
./example.sh > example.out

npx md-include .markdown.json

cargo set-version --bump patch

git add -u
git commit -m dist
git push

cargo +nightly publish
