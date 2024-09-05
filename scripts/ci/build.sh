#!/usr/bin/env bash
#
# This script builds the system
#

mode=${1}

if [ -z "${mode}" ]; then
    echo "Missing mode (debug|release)"
    exit 1
fi

if [ "${mode}" = "debug" ]; then
    mode_str=""
elif [ "${mode}" = "release" ]; then
    mode_str="--release"
else
    echo "Invalid mode '${mode}', must be 'debug' or 'release'"
    exit 2
fi

cargo --color always build ${mode_str} --all-features --tests --examples --bins
