FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin keeper_extractor
WORKDIR /keeper_extractor

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get --assume-yes install \
        make \
        libpq5 \
        libpq-dev \
        -qqy \
        --no-install-recommends
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /keeper_extractor/target/release/keeper_extractor /keeper_extractor/keeper_extractor
WORKDIR /keeper_extractor/
EXPOSE 8088


CMD ["/keeper_extractor/keeper_extractor"]

