FROM ubuntu:latest AS main

WORKDIR /mnt

RUN apt update

RUN apt install -y cmake libopus-dev pkg-config libssl-dev

COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

RUN $HOME/.cargo/bin/cargo build --release

WORKDIR /mnt/target/release
CMD ["bot"]