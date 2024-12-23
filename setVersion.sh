#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <new_version>"
  exit 1
fi

new_version=$1

sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
