FROM ubuntu:latest AS main

WORKDIR /mnt

RUN apt update

RUN apt install -y cmake libopus-dev pkg-config libssl-dev curl

COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY yt-dlp /usr/local/bin/yt-dlp

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . "$HOME/.cargo/env" && cargo build --release

WORKDIR /mnt/target/release

CMD ["./bot"]