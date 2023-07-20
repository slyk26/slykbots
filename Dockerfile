FROM rust:latest AS builder
RUN update-ca-certificates

# Create appuser
ENV USER=murkov
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /murkov

COPY ./ .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update
RUN apt-get -y install libopus-dev
RUN apt-get -y install cmake
RUN apt-get -y install protobuf-compiler
RUN apt-get -y install build-essential
RUN apt-get -y install autoconf automake
RUN apt-get -y install libtool m4 ffmpeg
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
RUN chmod a+rx /usr/local/bin/yt-dlp

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /murkov

# Copy build
COPY --from=builder /murkov/target/release/murkov ./

# Use an unprivileged user.
USER murkov:murkov