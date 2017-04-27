# shasum
[![Build Status](https://travis-ci.org/smirnovvad/shasum.svg?branch=master)](https://travis-ci.org/smirnovvad/shasum)

shasum implemented in rust using octavo lib

Calculate sha hashes for files

also can print hash for each file in directory

>USAGE:
>    shasum [OPTIONS] <INPUT>
>
>FLAGS:
>    -h, --help       Prints help information
>    -V, --version    Prints version information
>
>OPTIONS:
>    -a <algorithm>        [default: 1]  [values: 1, 224, 256, 384, 512, 512224, 512256]
>
>ARGS:
>    <INPUT>    Sets the input file to use
