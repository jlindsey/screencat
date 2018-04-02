#!/bin/bash

# Build script shamelessly stolen from ripgrep :)

cargo build --target $TARGET --release

build_dir=$(mktemp -d 2>/dev/null || mktemp -d -t tmp)
out_dir=$(pwd)
name="${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"
mkdir "$build_dir/$name"

cp target/$TARGET/release/$PROJECT_NAME "$build_dir/$name/"
strip $build_dir/$name/$PROJECT_NAME
cp LICENSE "$build_dir/$name/"

pushd $build_dir
tar czf "$out_dir/$name.tar.gz" *
popd