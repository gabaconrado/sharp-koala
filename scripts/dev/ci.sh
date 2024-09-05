#!/usr/bin/env bash
#
# This script runs all validations that are run in CI development workflow at once
#

./scripts/ci/build.sh "debug"
./scripts/ci/lint.sh
./scripts/ci/test.sh
