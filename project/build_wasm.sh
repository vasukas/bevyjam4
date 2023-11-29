#!/bin/bash
#
# Local build for WebAssembly (tested only on Linux)
#
# Builds crate for wasm.
# Optionally runs it on local server and opens it in browser.
# Optionally uploads build to itch.io
# If you don't use itch, files ready for upload should be in "target/wasm_package".
#
# Options:
#     --run [ADDR:PORT]
#         Starts python3 HTTP server at specified address.
#         If argument is not specified, default localhost address is used.
#         Press Ctrl+C to stop server and go to deploying to itch.
#
#     --browser [PATH]
#         Open game in specified browser (only if used together with --run).
#         If no path is given, uses xdg-open to open with default browser.
#
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
#
#     --flags <STRING>
#         String is passed to "cargo build", unquoted (yes, entire string must be quoted!).
#         If this option is not used, "--release --no-default-features" will be passed to cargo.
#
#     --html <FILE>
#         Use specified HTML file instead of generating new one.
#
# Prerequisites:
# * bash, cargo, sed
# * wasm-bindgen (install with "cargo install wasm-bindgen-cli")
# * python3 (for --run option)
# * butler (for --itch option)
#
# Butler configuration:
# * install: https://itch.io/docs/butler/installing.html
#     on Arch it's available as AUR package
# * run "butler login" (needed only once)
# * that's it!
#

set -e  # exit on errors

# dir re-created in the target directory on each build, contains generated files
output_dir=wasm_package

# default value for --run option
default_run_address=127.0.0.1:8000

# default value for --flags
default_cargo_flags="--release --no-default-features"

# file which allows audio to resume on user interaction if autoplay not allowed
# TODO: make this configurable
audio_autoplay_script="project/web-audio-autoplay.js"

#
# PARSE ARGUMENTS
#

run_address=
run_browser=
itch_deploy=
cargo_flags=$default_cargo_flags
html_source=

# Notes for bash non-gurus (like myself a year later):
#   This iterates over all arguments, by reading first one, matching it ('case'),
#    then removing it together with all options ('shift').
#   Comparison to -* checks if string begins with '-' and any characters later,
#    so argument can follow after option.
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo "(build_wasm) Read the description in the script itself"
            exit 1
        ;;
        
        --run)
            if [[ -z "$2" ]] || [[ "$2" == -* ]]; then
                run_address=$default_run_address
            else
                run_address=$2
                shift
            fi
            shift
        ;;

        --browser)
            if [[ -z "$2" ]] || [[ "$2" == -* ]]; then
                run_browser=xdg-open
            else
                run_browser=$2
                shift
            fi
            shift
        ;;
        
        --itch)
            itch_deploy=$2
            shift 2
        ;;

        --flags)
            cargo_flags=$2
            shift 2
        ;;

        --html)
            html_source=$2
            shift 2
        ;;
        
        *)
            echo "(build_wasm) ERROR: unknown script option \"$1\""
            exit 1
        ;;
    esac
done

#
# BUILD CRATE
#

# build crate
echo "(build_wasm) INFO: running cargo with flags: $cargo_flags"
cargo build $cargo_flags --target wasm32-unknown-unknown
echo "(build_wasm) INFO: cargo ok"

# extract project name from Cargo.toml
#   copied from https://github.com/team-plover/warlocks-gambit/blob/main/scripts/wasm_build.sh
project_name="$(cargo metadata --no-deps --format-version 1 |
    sed -n 's/.*"name":"\([^"]*\)".*/\1/p')"
project_version="$(cargo metadata --no-deps --format-version 1 |
    sed -n 's/.*"version":"\([^"]*\)".*/\1/p')"
# extract name of the target directory
target_dir="$(cargo metadata --format-version 1 |
    sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p')"

# get name of the built file
wasm_file="$target_dir/wasm32-unknown-unknown/release/$project_name.wasm"
if [ ! -e "$wasm_file" ]; then
    echo "(build_wasm) ERROR: script is broken, it expects file to exist: $wasm_file"
    exit 1
fi

echo "(build_wasm) INFO: project_name = \"$project_name\""
echo "(build_wasm) INFO: project_version = \"$project_version\""
echo "(build_wasm) INFO: target_dir = \"$target_dir\""
echo "(build_wasm) INFO: wasm_file = \"$wasm_file\""

# find bindgen
#   copied from https://github.com/team-plover/warlocks-gambit/blob/main/scripts/wasm_build.sh
BINDGEN_EXEC_PATH="${CARGO_HOME:-$HOME/.cargo}/bin/wasm-bindgen"
if [ ! -e "$BINDGEN_EXEC_PATH" ]; then
    echo "(build_wasm) ERROR: wasm-bindgen not found at \"$BINDGEN_EXEC_PATH\""
    echo "(build_wasm) Run \"cargo install wasm-bindgen-cli\" to install it"
    exit 1
fi

# re-create output directory
output_dir=$target_dir/$output_dir
[ ! -e "$output_dir" ] || rm -r "$output_dir"

# generate js
echo "(build_wasm) INFO: running bindgen..."
"$BINDGEN_EXEC_PATH" --no-typescript \
    --out-dir "$output_dir" --target web "$wasm_file"
echo "(build_wasm) INFO: bindgen ok"

# copy autoplay javascript
cp "$audio_autoplay_script" "$output_dir/audio_autoplay.js"

# create HTML
html_file="$output_dir/index.html"
if [[ ! -z "$html_source" ]]; then
    cp "$html_source" "$html_file"
else
    cat > "$html_file" <<EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
</head>
<body>
    <script>
        document.addEventListener("contextmenu", function (e) {
            e.preventDefault(); // for right-click to work
        }, false);
    </script>
    <script type="module">
        import './audio_autoplay.js'
        import init from './$project_name.js'
        init();
    </script>
</body>
</html>
EOF
# note: this EOF must be untabbed
fi

# copy assets
cp -r assets "$output_dir/assets"

echo "(build_wasm) INFO: All files required to run wasm build are in "$output_dir""

#
# OPTIONS
#

# run HTTP server & browser
if [[ ! -z "$run_address" ]]; then
    link="http://$run_address/index.html"
    echo "(build_wasm) INFO: running HTTP server at \"$run_address\" (page is at \"$link\")"

    addr=`echo $run_address | sed "s/:.*//"`
    port=`echo $run_address | sed "s/.*://"`
    python3 -m http.server --bind $addr --directory "$output_dir" $port &
    server_job=$!

    if [[ ! -z "$run_browser" ]]; then
        sleep 2s  # sometimes browser opens faster than server starts ¯\_(ツ)_/¯
        "$run_browser" "$link"
    fi

    # wait for Ctrl-C; source: https://stackoverflow.com/a/58508884
    ( trap exit SIGINT ; read -r -d '' _ </dev/tty )

    kill $server_job
fi

# deploy to itch
if [[ ! -z "$itch_deploy" ]]; then
    echo "(build_wasm) INFO: deploying to itch.io: $itch_deploy"

    butler push --if-changed --userversion=$project_version \
        "$output_dir" "$itch_deploy"
fi
