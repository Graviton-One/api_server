FROM rustlang/rust:nightly-buster as builder

RUN USER=root cargo new --bin async_scheduler
WORKDIR /async_scheduler

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
RUN rm ./target/release/deps/async_scheduler*
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

COPY --from=builder /async_scheduler/target/release/async_scheduler /async_scheduler/async_scheduler
WORKDIR /async_scheduler/

CMD ["/async_scheduler/async_scheduler"]

