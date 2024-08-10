#!/usr/bin/env bash

# This utility uses real `echo` to generate some expected outputs
# For use in integration testing of our reimplementation
# It expects to be run from the project root.

OUTDIR="./tests/expected"
[[ ! -d "${OUTDIR}" ]] && mkdir -p "${OUTDIR}"

echo    "Hello there" > "${OUTDIR}/hello1.txt"
echo    "Hello" "there" > "${OUTDIR}/hello2.txt"
echo -n "Hello there" > "${OUTDIR}/hello1.n.txt"
echo -n "Hello" "there" > "${OUTDIR}/hello2.n.txt"
