#!/bin/zsh

for dir in ./{hal/,pac/*}
do
    pushd $dir
    cargo clean
    popd
done
