#!/usr/bin/env bash
DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd $DIR
mkdir ./data/ext-workdir

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

#if [ ! -f ./data/ext-workdir/fuse-archive.tar ]; then
#    cd ./data/datasets/
#    time tar -cf ../ext-workdir/fuse-archive.tar .
#    cd -
#fi
#mkdir ./data/mountpoints/fuse-archive-tar/
#time fuse-archive ./data/ext-workdir/fuse-archive.tar ./data/mountpoints/fuse-archive-tar/

# **DISABLED** - also too slow
#cd ./data/datasets/
#if [ ! -f ./data/ext-workdir/fuse-archive.tar.zst ]; then
#    time bash -c 'tar -cf - . | zstd -1 - -o ../ext-workdir/fuse-archive.tar.zst'
#fi
#cd -
#mkdir ./data/mountpoints/fuse-archive-tar-zst/
#time fuse-archive ./data/ext-workdir/fuse-archive.tar.zst ./data/mountpoints/fuse-archive-tar-zst/

# btrfs-fuse is broken - ERROR: failed to scan device /dev/nvme0n1p3: -13
device=""
#mkdir ./data/mountpoints/btrfs-fuse
#sudo mount $device ./data/mountpoints/btrfs-fuse
#sudo chmod -R 777 ./data/mountpoints/btrfs-fuse/
#if [ ! -f ./data/mountpoints/btrfs-fuse/25G-null.bin ]; then
#    cp -r ./data/datasets/* ./data/mountpoints/btrfs-fuse/
#    sudo umount ./data/mountpoints/btrfs-fuse/
#fi
#btrfs-fuse $device ./data/mountpoints/btrfs-fuse
