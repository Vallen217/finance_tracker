#!/bin/bash

# get the working directory of finance_tracker.
FNTR_DIR="$(dirname "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)")"

cd "$FNTR_DIR" || exit
cargo build --release

cd "$HOME" || exit
cargo install --path "$FNTR_DIR"

# create the directory which will store files generated by the finance_tracker.
mkdir -p "$HOME/Documents/Finance/Records"

# copy the launch script to bin
# so that specifying the script path is no longer necessary when executing.
chmod +x "$FNTR_DIR/scripts/fntr.sh"
sudo cp "$FNTR_DIR/scripts/fntr.sh" "/usr/local/bin/"
