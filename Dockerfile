FROM debian:7
MAINTAINER stefan@estada.ch

ENV TARGET_UID="1000" TARGET_GID="1000"

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl sudo file build-essential && mkdir /compile && curl -sSf https://static.rust-lang.org/rustup.sh | sh

VOLUME /compile
WORKDIR /compile

CMD cargo build --release ; chown -R ${TARGET_UID}:${TARGET_GID} target
