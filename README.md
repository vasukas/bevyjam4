# Overview

Game for [Bevy Jam #4](https://itch.io/jam/bevy-jam-4)

# Build

Release build should be done with `--no-default-features` (to disable dynamic linking).

Can be built for WebAssembly by `project/build_wasm.sh` script.

If build fails on bindgen stage, you may need to update it: `cargo install wasm-bindgen-cli`.

## Temporary fixes for WASM build

Had to enable bevy "webgl2" feature to disable WebGPU - same errors as in https://github.com/bevyengine/bevy/issues/10477.



# License

## Code

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

All licenses permit use in typical **open-source** project, including commercial ones.

## Assets

Except where noted otherwise (in individiual LICENSE or CREDITS files), all assets are made by me and
are licensed under Creative Commons Zero (CC0), http://creativecommons.org/publicdomain/zero/1.0/.

## Note

Some individual code snippets, libraries and assets used in this project are licensed differently, with license mentioned only in their respective file.
