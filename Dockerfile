FROM rust:1.43-slim

WORKDIR /srv

RUN rustup component add rustfmt

COPY . .
# RUN cargo fetch \
#     && cargo build --all --tests
# CMD cargo run
