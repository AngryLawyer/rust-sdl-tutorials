#!/bin/bash
# Bash script will eventually be replaced with a make file

TARGET=$1

# Check we need a library??
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

# Make the bin directory
if [ ! -d "bin" ]; then
    mkdir bin
fi
# Make the libs directory
if [ ! -d "libs" ]; then
    mkdir libs
fi
# Scrape them
rm libs/*
mv rust-sdl/*.dummy libs
mv rust-sdl/*.so libs

# Check what project we want to build, or all of them
# Hack it for now
cd src/$TARGET
    ./build.sh
cd -
