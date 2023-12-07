FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /murkov

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /murkov/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN apt-get update
RUN apt-get -y install cmake
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin murkov

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /murkov
COPY --from=builder /murkov/target/release/murkov /usr/local/bin

RUN apt-get update
RUN apt-get -y install libopus-dev cmake protobuf-compiler build-essential autoconf automake libtool m4 ffmpeg curl python3
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
RUN chmod a+rx /usr/local/bin/yt-dlp
ENTRYPOINT ["/usr/local/bin/murkov"]