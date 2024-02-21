#!/bin/bash

if [ "$1" == "-b" ] || [ "$1" == "--build" ]; then
    echo "Building..."

    cd "$(dirname "$0")" || exit
    cd ../map-editor/ || exit

    wasm-pack build --target web
fi

cd "$(dirname "$0")" || exit

ROOT="../../"

echo "Copying pkg build..."
cp -r "${ROOT}/Editor/map-editor/pkg" public/scripts/
echo -e "\x1b[32mPkg build copied\x1b[0m"
