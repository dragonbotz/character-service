# Character service Dockerfile
#
# Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
FROM ubuntu:22.04

SHELL ["/bin/bash", "-c"]

RUN apt update && apt install -y \
	curl \
	build-essential \
	git \
	pkg-config \
	libssl-dev

# install Rust tools
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN source $HOME/.cargo/env

RUN $HOME/.cargo/bin/cargo --help

WORKDIR app

# clone service
RUN git clone --recurse-submodules https://github.com/dragonbotz/character-service.git
WORKDIR character-service

RUN $HOME/.cargo/bin/cargo build --release

CMD ["./target/release/character-service"]
