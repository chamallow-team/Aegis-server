#!/bin/bash

# check if 'tailwindcss' is available
if ! [ -x "$(command -v tailwindcss)" ]; then
  echo 'Error: tailwindcss is not installed.' >&2
  exit 1
fi

cd "$(dirname "$0")"
tailwindcss -o public/style.css --watch