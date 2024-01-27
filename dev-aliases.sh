#!/usr/bin/env bash
# Just some convenience functions
# source with `source` or `.` and enjoy!

alias rest="cargo test"
alias clippy="cargo clippy"

# This one both quiets the normal output with -q
# and uses the -- seperater to ensure arguments
# are passed to the command line program
alias run="cargo run -q --"
