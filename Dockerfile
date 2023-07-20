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
RUN add-apt-repository --yes ppa:tomtomtom/yt-dlp
RUN apt-get update
RUN apt-get -y install yt-dlp

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /murkov

# Copy build
COPY --from=builder /murkov/target/release/murkov ./

# Use an unprivileged user.
USER murkov:murkov