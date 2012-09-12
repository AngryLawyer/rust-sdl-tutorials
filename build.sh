#!/bin/bash
# Bash script will eventually be replaced with a make file

# Chcek if we need to update
cd rust-sdl
    git pull
    ./configure
    make
    RESULT=$?
    if [ $RESULT -ne 0 ] ; then
        echo "Failed to make dependancies"
        cd -
        exit $RESULT
    fi
cd -
