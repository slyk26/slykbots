# syntax=docker/dockerfile:1.3-labs
FROM rust:latest AS builder
RUN update-ca-certificates

RUN apt-get update
RUN apt-get -y install libopus-dev cmake protobuf-compiler build-essential autoconf automake libtool m4 ffmpeg curl python3
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
RUN chmod a+rx /usr/local/bin/yt-dlp

COPY Cargo.toml Cargo.lock /src/

# We create a new lib and then use our own Cargo.toml
RUN cargo new /temp/murkov
COPY Cargo.toml /temp/murkov/


WORKDIR /temp/murkov
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

COPY ./ /temp/murkov


RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  # update timestamps to force a new build
  touch /temp/murkov/src/main.rs
  cargo build --release
EOF

CMD ["/temp/target/release/murkov"]

FROM ubuntu:latest

RUN apt-get update
RUN apt-get -y install libopus-dev cmake protobuf-compiler build-essential autoconf automake libtool m4 ffmpeg curl python3

COPY --from=builder /usr/local/bin/yt-dlp /usr/local/bin/yt-dlp

COPY --from=builder /temp/target/release/my-murkov /murkov

WORKDIR /murkov

CMD ["/murkov"]