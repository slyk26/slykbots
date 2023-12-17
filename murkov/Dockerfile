FROM rust:latest AS chef
# gordon ramsey - you donkey
RUN cargo install cargo-chef
# migrating command because via migrate!() macro its fucked and always errors because of workspace
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

FROM chef AS planner
COPY murkov ./murkov
COPY ../shared ./shared
WORKDIR murkov
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner murkov/recipe.json murkov/recipe.json
COPY --from=planner ../shared shared
RUN apt-get update
RUN apt-get -y install cmake
RUN cargo chef cook --release --recipe-path murkov/recipe.json
COPY . .
RUN cargo build --release --bin murkov --package slykbots

FROM debian:bookworm-slim AS runtime
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/
COPY --from=builder /target/release/murkov /usr/local/bin
COPY ../migrations /usr/local/bin/migrations
COPY murkov/murkov-startup.sh /usr/local/bin/murkov-startup
RUN chmod +x /usr/local/bin/murkov-startup

RUN apt-get update
RUN apt-get -y install libopus-dev cmake protobuf-compiler build-essential autoconf automake libtool m4 ffmpeg curl python3
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
RUN chmod a+rx /usr/local/bin/yt-dlp
ENTRYPOINT ["/usr/local/bin/murkov-startup"]