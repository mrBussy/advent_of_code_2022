FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install git

RUN rustup toolchain install nightly-x86_64-unknown-linux-gnu
RUN rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu

RUN cargo install cargo-aoc
