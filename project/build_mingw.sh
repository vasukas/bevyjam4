#!/bin/bash
#
# Copy-pasted from build_wasm.sh
#
# Options:
#     --itch <USER/PROJECT:CHANNEL>
#         Pushes game to itch.io using butler (official CLI tool).
#
#         USER is your itch.io username;
#         PROJECT is a project name (as in URL);
#         CHANNEL is a channel name, whatever you want.
#            (see https://itch.io/docs/butler/pushing.html#channel-names)
#         i.e. "username/awesome-game:web-beta"
#
#         Also sends --userversion with value read from cargo package version.

set -xe  # exit on errors

# dir re-created in the target directory on each build, contains generated files
output_dir=mingw_package
TARGET="x86_64-pc-windows-gnu"

# default value for --flags
cargo_flags="--release --target $TARGET --no-default-features --features multi-threaded"

itch_deploy=

while [[ $# -gt 0 ]]; do
    case $1 in
        --itch)
            itch_deploy=$2
            shift 2
        ;;

        *)
            echo "(build_wasm) ERROR: unknown script option \"$1\""
            exit 1
        ;;
    esac
done

# build crate
cargo build $cargo_flags

# extract project name from Cargo.toml
project_name="$(cargo metadata --no-deps --format-version 1 |
    sed -n 's/.*"name":"\([^"]*\)".*/\1/p')"
project_version="$(cargo metadata --no-deps --format-version 1 |
    sed -n 's/.*"version":"\([^"]*\)".*/\1/p')"
# extract name of the target directory
target_dir="$(cargo metadata --format-version 1 |
    sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p')"

# re-create output directory
output_dir=$target_dir/$output_dir
[ ! -e "$output_dir" ] || rm -r "$output_dir"
mkdir -p "$output_dir"

# copy binary
cp "$target_dir/$TARGET/release/$project_name.exe" "$output_dir"

# copy assets
cp -r assets "$output_dir/assets"

# deploy to itch
if [[ ! -z "$itch_deploy" ]]; then
    echo "(build_wasm) INFO: deploying to itch.io: $itch_deploy"

    butler push --if-changed --userversion=$project_version \
        "$output_dir" "$itch_deploy"
fi
