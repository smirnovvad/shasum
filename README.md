# shasum
[![Build Status](https://travis-ci.org/smirnovvad/shasum.svg?branch=master)](https://travis-ci.org/smirnovvad/shasum)

shasum implemented in rust.

Print SHA checksums from STDIN, input file or directory.

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

## Examples
```
→ shasum ./LICENSE -a 256
b40930bbcf80744c86c46a12bc9da056641d722716c378f5659b9e555ef833e1    ./LICENSE
```
```
→ shasum ./
9b590aaaa51c06b11a30491cab86575dd462670e   ./.gitignore
ba6d16a4e988679755ad199ac081020f5b885451   ./.travis.yml
1c809b2457dcc9fa32940180bd713aae803c0f15   ./Cargo.lock
2d2d85d7b21778fb6369b3321340138ecfa3a01d   ./Cargo.toml
92170cdc034b2ff819323ff670d3b7266c8bffcd   ./LICENSE
8eec4953d6cba08088647cff4d5b64e9153ec530   ./README.md
```
## Install
```
cargo install shasum
```
