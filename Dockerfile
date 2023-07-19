FROM rust:latest AS builder
RUN update-ca-certificates
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

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

RUN cargo +nightly build --release -Z sparse-registry

FROM gcr.io/distroless/cc

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /murkov

# Copy build
COPY --from=builder /murkov/target/release/murkov ./

# Use an unprivileged user.
USER murkov:murkov