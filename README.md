# lasttest

lasttest is a load generator written in rust

It is designed for NUMA-Systems

## Run local

```
make run
```

## Adapt deploy
Edit this line in the Makefile:

```Makefile
deploy: compileDocker
	ssh mathhsr 'cat > lasttest' < target/release/lasttest
```

and replace the `mathhsr` with your host.

# Errors

## libc not found / too old

```bash
./lasttest: /lib64/libc.so.6: version `GLIBC_2.18' not found (required by ./lasttest)
```

If you encounter this error, use the docker container to build lasttest with an older libc.

