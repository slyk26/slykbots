FROM rust:latest AS builder
RUN update-ca-certificates
RUN apt-get update && apt-get -y install cmake protobuf-compiler

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

FROM gcr.io/distroless/cc

RUN add-apt-repository ppa:tomtomtom/yt-dlp
RUN apt-get update && apt-get -y install cmake protobuf-compiler essential autoconf automake libtool m4 ffmpeg yt-dlp

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /murkov

# Copy build
COPY --from=builder /murkov/target/release/murkov ./

# Use an unprivileged user.
USER murkov:murkov