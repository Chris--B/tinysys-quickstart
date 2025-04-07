#!/bin/bash

TINYSYS_ROOT=${TINYSYS_ROOT:-../tinysys}
TINYSYS_SDCARD=${TINYSYS_SDCARD:-$TINYSYS_ROOT/software/emulator/sdcard/}

function usage() {
    cat <<EOF
Usage: $(basename "$0") <file>

Arguments:
  file              Path to a file to copy to the tinysys emuilator sd card.
                    Directories are not supported.


Environment Variables:
  TINYSYS_ROOT      Override the path to the tinysys repo root.
                    Default: "../tinysys".
                    Current: "$TINYSYS_ROOT".

  TINYSYS_SDCARD    Override the path to the tinysys emulator sdcard.
                    Setting this completely discards TINYSYS_ROOT.
                    Default: "\$TINYSYS_ROOT/software/emulator/sdcard/"
                    Current: "$TINYSYS_SDCARD".


Example:
  $0 foo.elf
EOF
    exit 1
}

if [ -d "$TINYSYS_SDCARD" ] ; then
    if [ $# -eq 0 ]; then
        echo "Error: Not given a file to copy to the sdcard (directories are not supported)".
        echo
        usage
        exit 1
    fi

    set -e

    # Copy the binary we're being given to the sdcard
    file=$1
    cp -v $file $TINYSYS_SDCARD

    # Make sure the filename ends in ".elf" or else the emulator won't run it
    if [[ "$file" != *.elf ]]; then
        pushd $TINYSYS_SDCARD > /dev/null
        echo "+ cd $TINYSYS_SDCARD"
        elf=$(basename "$file")
        mv -v "./$elf" "./$elf.elf"
    fi


    exit 0
else
    echo "Trying to use sdcard at $TINYSYS_SDCARD, but the directory doesn't seem to exist."
    echo
    usage
    exit 2
fi
