#!/usr/bin/env bash

set -e

cd "${0%/*}" # set a consistent directory to be the location of this script

# Commands to run after the Container is started

# git
git config --global --add safe.directory "$1"

