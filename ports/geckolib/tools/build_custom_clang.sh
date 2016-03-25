#!/bin/bash

# Run in the tools directory.
cd "$(dirname $0)"

# Don't run twice.
if [ -d llvm ]; then
  echo "llvm directory already exists."
  exit 1
fi

# Download and build a custom llvm
git clone https://github.com/llvm-mirror/llvm
cd llvm
git checkout release_38
cd tools
git clone https://github.com/llvm-mirror/clang
cd clang
git checkout release_38
cd ../.. # llvm root dir
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo ..
make -j8
