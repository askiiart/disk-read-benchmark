#!/usr/bin/env bash
DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd $DIR

if [ ! -f ./data/ext-workdir/dwarfs ]; then
    time mkdwarfs -l 1 -i ./data/datasets/ -o ./data/ext-workdir/dwarfs
fi
mkdir ./data/mountpoints/dwarfs/
time dwarfs ./data/ext-workdir/dwarfs ./data/mountpoints/dwarfs/

# **DISABLED** - far too slow
#cd ./data/datasets/
#if [ ! -f ./data/ext-workdir/fuse-archive.tar.gz ]; then
#    time tar -czf ../ext-workdir/fuse-archive.tar.gz .
#fi
#cd -
#mkdir ./data/mountpoints/fuse-archive-tar-gz/
#time fuse-archive ./data/ext-workdir/fuse-archive.tar.gz ./data/mountpoints/fuse-archive-tar-gz/

cd ./data/datasets/
if [ ! -f ./data/ext-workdir/fuse-archive.tar ]; then
    time tar -cf ../ext-workdir/fuse-archive.tar .
fi
cd -
mkdir ./data/mountpoints/fuse-archive-tar/
time fuse-archive ./data/ext-workdir/fuse-archive.tar ./data/mountpoints/fuse-archive-tar/

# **DISABLED** - also too slow
#cd ./data/datasets/
#if [ ! -f ./data/ext-workdir/fuse-archive.tar.zst ]; then
#    time bash -c 'tar -cf - . | zstd -1 - -o ../ext-workdir/fuse-archive.tar.zst'
#fi
#cd -
#mkdir ./data/mountpoints/fuse-archive-tar-zst/
#time fuse-archive ./data/ext-workdir/fuse-archive.tar.zst ./data/mountpoints/fuse-archive-tar-zst/
