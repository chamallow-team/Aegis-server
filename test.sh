#!/bin/bash

all() {
  echo "\x1b[32mTesting all the packages\x1b[0m"
  cargo test
}

specific() {
  echo -e "\x1b[32mTesting the specific package\x1b[0m"
  cargo test --package "$1"
}

wasm() {
  echo -e "\x1b[32mTesting all the packages with is targeted for wasm\x1b[0m"
  cd "$(dirname "$0")" || exit
  cd Editor/map_view-editor || exit

  IS_HEADLESS=""
  if [ "$2" = "--headless" ] || [ "$2" == "-hl" ]; then
    IS_HEADLESS="--headless"
  fi

  if [ "$1" = "chrome" ] || [ "$1" == "--chrome" ]; then
    echo -e "\x1b[32mTesting with chrome\x1b[0m"
    wasm-pack test --chrome $IS_HEADLESS
  elif [ "$1" = "firefox" ] || [ "$1" = "--firefox" ]; then
    echo -e "\x1b[32mTesting with firefox\x1b[0m"
    wasm-pack test --firefox $IS_HEADLESS
  else
    echo -e "\x1b[32mTesting with default navigator\x1b[0m"
    wasm-pack test $IS_HEADLESS
  fi
}

help() {
  echo "Usage: test.sh [option] [package] [navigator]"
  echo "Options:"
  echo "  -h, --help      Show this help message and exit"
  echo "  -a, --all       Test all the packages"
  echo "  -w, --wasm      Test the packages that are targeted for wasm"
  echo "  -p, --package   Test the specific package"
  echo "  -hl, --headless Run the tests in headless mode"
  echo "  [package]       The package to test"
  echo "  [navigator]     The navigator to use for the wasm tests"
  echo "                  (chrome, firefox, or default)"
  echo "Example:"
  echo "  test.sh -a"
  echo "  test.sh --wasm"
  echo "  test.sh --package map-editor"
  echo "  test.sh --wasm --headless"
  echo "  test.sh --wasm chrome"
  echo "  test.sh --wasm firefox"
  echo "  test.sh --wasm default"
}

case "$1" in
  "-h" | "--help")
    help
    ;;
  "-a" | "--all")
    all
    ;;
  "-w" | "--wasm")
    wasm "$2" "$3"
    ;;
  "-p" | "--package")
    specific "$2"
    ;;
  *)
    echo "Invalid option"
    help
    exit 1
    ;;
esac

exit 0
