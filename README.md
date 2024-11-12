# Read-only benchmark

This tests the latency, sequential read, and random read speeds of a variety of data.

## Installation

To install this, run the following:

```sh
git clone https://git.askiiart.net/askiiart/disk-read-benchmark
cd ./disk-read-benchmark/
cargo update
cargo install --path .
```

Make sure to generate and add the completions for your shell:

- bash: `disk-read-benchmark generate-bash-completions | source`
- zsh: `disk-read-benchmark generate-zsh-completions | source`
- fish: `disk-read-benchmark generate-fish-completions | source`

(note that this only lasts until the shell is closed)

## Running

The program will automatically generate all data used, except for the regular polygon data. Once the data is generated, stop the program with Ctrl+C, then run `prepare.sh` to archive and mount the data using [DwarFS](https://github.com/mhx/dwarfs), `tar`, and [`fuse-archive`](https://github.com/google/fuse-archive).

It will output its data at `./data/benchmark-data.csv` and `./data/bulk.csv` in these formats:

`benchmark-data.csv`:

```txt
filesystem dir,file path,sequential read time,sequential read latency,random read time,random read latency
```

`bulk.csv`:

```txt
filesystem dir,folder path,test type,time1,time2,time3,[...]
```
