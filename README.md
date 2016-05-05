# lasttest

lasttest is a load generator written in rust

It is designed for NUMA-Systems

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

# Errors

## libc not found / too old

```bash
./lasttest: /lib64/libc.so.6: version `GLIBC_2.18' not found (required by ./lasttest)
```

If you encounter this error, use the docker container to build lasttest with an older libc.

