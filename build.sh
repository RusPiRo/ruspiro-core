#!/bin/sh
#*********************************************************************************
# script to build the this crate
#*********************************************************************************
set +ev

if [ $# -eq 0 ] 
    then 
        echo "provide the target architecture to build for - 32 or 64"
        exit 1
fi

if [ $1 = "64" ]
    then
        # aarch64
        # use the right compiler toolchain prefix when building on travis
        if [ -z "$2" ]
            then
                PREFIX=aarch64-elf-
            else
                PREFIX=aarch64-linux-gnu-
        fi
        CFLAGS="-march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53"
        RUSTFLAGS="-C linker=${PREFIX}gcc -C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+neon -C link-arg=-nostartfiles -C opt-level=3 -C debuginfo=0"
        TARGET="aarch64-unknown-linux-gnu"
elif [ $1 = "32" ]
    then
        # aarch32
        # use the right compiler toolchain prefix when building on travis
        if [ -z "$2" ]
            then
                PREFIX=arm-eabi-
            else
                PREFIX=arm-linux-gnueabihf-
        fi
        CFLAGS="-mfpu=neon-fp-armv8 -mfloat-abi=hard -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53"
        RUSTFLAGS="-C linker=${PREFIX}gcc -C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+v8,+vfp3,+d16,+thumb2,+neon -C link-arg=-nostartfiles -C opt-level=3 -C debuginfo=0"
        TARGET="armv7-unknown-linux-gnueabihf"
else
    echo 'provide the archtitecture to be build. Use either "build.sh 32" or "build.sh 64".'
    exit 1
fi

export CFLAGS="${CFLAGS}"
export RUSTFLAGS="${RUSTFLAGS}"
export CC="${PREFIX}gcc"
export AR="${PREFIX}ar"
export TARGET="${TARGET}"

cargo xbuild --target ${TARGET} --release
