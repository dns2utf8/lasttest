TARGET_HOST="mathhsro"


default: compile
	# done

run: clean
	cargo run --release local all

compile:
	cargo build --release

compileDocker: clean
	docker run --rm -it -v $$(pwd):/compile -e TARGET_UID="$$(id --user)" -e TARGET_GID="$$(id --group)" dns2utf8/rust-old

deploy: compileDocker
	ssh ${TARGET_HOST} 'cat > lasttest' < target/release/lasttest

clean:
	rm -rf target || true

dockerContainer:
	docker build --tag="dns2utf8/rust-old" .

musl:
	cargo build --release --target=x86_64-unknown-linux-musl
