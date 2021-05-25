FROM rustlang/rust:nightly as builder

WORKDIR /api
COPY . .
RUN cargo clean
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

COPY --from=builder /api/target/release/ftm_poller /api/ftm_poller
WORKDIR /api
EXPOSE 8000


CMD ["/api/ftm_poller"]
