# Builder image for compilation
FROM rust:1.42 as builder

WORKDIR /usr/src

RUN set -x && USER=root cargo new --bin sento-api

WORKDIR /usr/src/sento-api

COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Cache deps for faster build times
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build the code
RUN rm -rf ./target/release/deps/sento-api
RUN USER=root cargo build --release

# Final execution image
FROM debian:buster-slim

WORKDIR /usr/bin

# Copy the executable from the builder image
COPY --from=builder /usr/src/sento-api/target/release/sento-api .

RUN set -x && groupadd -r sento-api && useradd -r -g sento-api sento-api

USER sento-api

CMD ["./sento-api"]
