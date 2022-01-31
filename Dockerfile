# synax = docker/dockerfile:1.2
FROM rust:alpine3.15
ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /usr/src/app
#RUN apk add --no-cache musl-dev openssl-dev shared-mime-info sqlite-dev xz-dev zstd-dev
RUN apk add --no-cache musl-dev openssl-dev sqlite-dev xz-dev
COPY . .
RUN --mount=type=cache,target=/var/cache/buildkit \
    CARGO_HOME=/var/cache/buildkit/cargo \
    CARGO_TARGET_DIR=/var/cache/buildkit/target \
    cargo build --release --locked && \
    cp -v /var/cache/buildkit/target/release/rebuilderd-debian-buildinfo-crawler /

FROM alpine:3.15
RUN apk add --no-cache libgcc openssl sqlite-libs xz
COPY --from=0 \
    /rebuilderd-debian-buildinfo-crawler \
    /usr/local/bin/
VOLUME ["/data"]
WORKDIR /data
ENTRYPOINT ["rebuilderd-debian-buildinfo-crawler"]
