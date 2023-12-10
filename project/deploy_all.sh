#!/bin/bash

set -e

project/build_wasm.sh --itch vasukas/alien-overload:jam
project/build_linux.sh --itch vasukas/alien-overload:jam-linux
project/build_mingw.sh --itch vasukas/alien-overload:jam-mingw
