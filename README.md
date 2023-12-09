# Overview

Game for [Bevy Jam #4](https://itch.io/jam/bevy-jam-4).

Can be played at [itch.io](https://vasukas.itch.io/alien-overload).



# Build

Release build should be done with `--no-default-features` to disable dynamic linking.

Can be built for WebAssembly by `project/build_wasm.sh` script. See the file for details.

If build fails on bindgen stage, you may need to update it: `cargo install wasm-bindgen-cli`.



# License

## Code

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Assets

Except where noted below, all assets are made by me (vasukas) and are licensed under [Creative Commons Zero (CC0)](http://creativecommons.org/publicdomain/zero/1.0/).

Basic human model is made with Makehuman for Blender (bundled Makehuman assets are licensed under Creative Commons Zero (CC0)).
