FROM rust:1.72.0-alpine3.18 as musl-builder

RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app



FROM musl-builder as frontend-builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

RUN cargo new backend

COPY Cargo.lock Cargo.toml ./
COPY frontend /app/frontend
COPY common /app/common

WORKDIR /app/frontend

RUN trunk build --release




FROM musl-builder as backend-builder

RUN cargo new frontend

COPY Cargo.lock Cargo.toml ./
COPY common /app/common
COPY backend /app/backend

RUN cargo build --bin backend --target x86_64-unknown-linux-musl --release




FROM alpine:3.18 as web-app-runner

RUN addgroup -S nonroot && adduser -S nonroot -G nonroot

USER nonroot
WORKDIR /home/nonroot

COPY --chown=nonroot:nonroot --from=backend-builder /app/target/x86_64-unknown-linux-musl/release/backend /home/nonroot/backend
COPY --chown=nonroot:nonroot --from=frontend-builder /app/frontend/dist /home/nonroot/dist

ENV BACKEND_DIST_PATH="/home/nonroot/dist"

ENTRYPOINT [ "sh", "-c", "./backend" ]
