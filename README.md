# lasttest

lasttest is a load generator written in rust

It is designed for 64 bit NUMA-Systems

## Build status
Linux and Mac: [![Build Status](https://travis-ci.org/dns2utf8/lasttest.svg?branch=master)](https://travis-ci.org/dns2utf8/lasttest)

Windows: <a href="https://ci.appveyor.com/project/dns2utf8/lasttest" target="_blank"><img src="https://ci.appveyor.com/api/projects/status/github/dns2utf8/lasttest?svg=true" width="120px"></a>

## Run local

```
cargo run --release all
```

## Adapt deploy
Edit this line in the Makefile:

```Makefile
TARGET_HOST="huge.cluster.machine.edu"
```

and replace the `mathhsr` with your host.

# Contribute

Pull Requests are very welcome.

# Errors

## libc not found / too old

```bash
./lasttest: /lib64/libc.so.6: version `GLIBC_2.18' not found (required by ./lasttest)
```

If you encounter this error, use the docker container to build lasttest with an older libc.

