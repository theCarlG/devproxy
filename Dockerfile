ARG COMMIT

# Build stage
FROM rust:1.82.0 as builder

LABEL org.opencontainers.image.source https://gitea.in.carlg.tech/CarlG/devproxy

ARG GIT_SHA

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

WORKDIR /code
COPY .cargo Cargo.toml Cargo.lock build.rs ./
COPY src ./src
RUN GIT_SHA=${GIT_SHA} cargo build --target x86_64-unknown-linux-musl --release

# Run stage
FROM alpine:latest

LABEL org.opencontainers.image.source https://gitea.in.carlg.tech/CarlG/devproxy

ENV RUST_LOG=info

WORKDIR /

RUN apk --no-cache add ca-certificates

# Copy our build
COPY --from=builder /code/target/x86_64-unknown-linux-musl/release/devproxy /devproxy

# Use an unprivileged user.
USER 1000:1000

CMD ["/sonos-media-proxy"]
