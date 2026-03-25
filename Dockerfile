ARG RUST_VERSION=1.94

ARG APP_NAME=easter-quest

FROM docker.io/library/rust:${RUST_VERSION}-alpine AS build

ARG APP_NAME

WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=bind,source=rsrc,target=rsrc \
    --mount=type=bind,source=templates,target=templates \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM docker.io/library/alpine:3.18 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser

WORKDIR /app

COPY --from=build /bin/server /bin/

COPY migrations /app/migrations
COPY rsrc /app/rsrc
COPY templates /app/templates

EXPOSE 8000

CMD ["/bin/server"]
