#!/bin/sh

scriptDir="`dirname "$0"`"
DYLD_LIBRARY_PATH="$scriptDir/../Resources/libs/" "$scriptDir/cat-chaser" \
    "$scriptDir/../Resources/assets/"
