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

FROM gcr.io/distroless/cc

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /bin/bash /bin/bash

WORKDIR /murkov

# Copy build
COPY --from=builder /murkov/target/release/murkov ./

# Use an unprivileged user.
USER murkov:murkov