FROM rust:latest AS chef
# gordon ramsey - you donkey
RUN cargo install cargo-chef
# migrating command because via migrate!() macro its fucked and always errors because of workspace
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

FROM chef AS planner
COPY rpp ./rpp
COPY ../shared ./shared
WORKDIR rpp
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner rpp/recipe.json rpp/recipe.json
COPY --from=planner ../shared shared
RUN apt-get update
RUN apt-get -y install cmake
RUN cargo chef cook --release --recipe-path rpp/recipe.json
COPY . .
RUN cargo build --release --bin rpp --package rpp

FROM debian:bookworm-slim AS runtime
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/
COPY --from=builder /target/release/rpp /usr/local/bin
COPY ../migrations /usr/local/bin/migrations
COPY rpp/rpp-startup.sh /usr/local/bin/rpp-startup
RUN chmod +x /usr/local/bin/rpp-startup

RUN apt-get update
RUN apt-get -y install ca-certificates
ENTRYPOINT ["/usr/local/bin/rpp-startup"]